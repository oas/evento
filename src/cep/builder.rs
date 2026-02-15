use std::collections::HashMap;
use std::sync::Arc;

/// Internal state index in the compiled NFA-like graph.
pub type StateId = usize;

/// Deliberately small event payload enumeration to keep predicate behavior predictable.
#[derive(Clone, Debug, PartialEq)]
pub enum EventValue {
    Int(i64),
    Float(f64),
    Text(String),
}

/// An event consumed by the CEP engine. It serves as a small envelope with three concerns:
/// - **when** it happened (`ts_millis`)
/// - **what category** it belongs to (`kind`)
/// - **details** used by predicates (`payload`)
#[derive(Clone, Debug)]
pub struct Event {
    /// Timestamp, used for `within` checks and state expiration.
    pub ts_millis: i64,
    /// Coarse-grained event category used for transition pre-filtering.
    pub kind: String,
    /// Flexible per-event data used by predicates.
    pub payload: HashMap<String, EventValue>,
}

/// A single instance of a pattern (a partial match).
///
/// `ActiveMatch` means "for one pattern and one partition, we have already
/// matched some prefix and are currently waiting in `state` for the next step".
///
/// If one active path is in state `S1` and event `E2` arrives, and `S1` has two
/// outgoing transitions that both accept `E2`, the runtime creates two children:
/// - child A follows transition `S1 -> S2`
/// - child B follows transition `S1 -> S3`
#[derive(Clone)]
struct ActiveMatch {
    /// Current NFA state for this pattern instance.
    state: StateId,
    /// Timestamp of the first event that started this path.
    started_at: i64,
    /// Timestamp of the most recently accepted event on this path.
    /// Used for `within` window checks and state-level expiration.
    last_ts: i64,
    /// Ordered events accepted by this path so far.
    ///
    /// Events are reference-counted (`Arc`) so sibling paths can share event objects
    /// after fork without copying event payloads.
    events: Vec<Arc<Event>>,
    /// Human-readable transition labels taken by this path, in order.
    ///
    /// Each element corresponds to one accepted transition.
    step_names: Vec<String>,
}

/// Buckets pattern instances by their current state.
///
/// This is an internal indexing layer. Instead of scanning all active paths,
/// event processing touches only buckets that can consume `event.kind`.
#[derive(Default)]
struct ActiveStateStore {
    buckets: HashMap<StateId, Vec<ActiveMatch>>,
}

impl ActiveStateStore {
    /// Returns a mutable bucket for `state`, creating it if absent.
    fn bucket_mut(&mut self, state: StateId) -> &mut Vec<ActiveMatch> {
        self.buckets.entry(state).or_default()
    }

    /// Removes and returns the entire bucket for `state`.
    ///
    /// Used by the runtime to process one state bucket in isolation, then put it back.
    fn take_bucket(&mut self, state: StateId) -> Vec<ActiveMatch> {
        // Remove-and-process avoids aliasing mutable borrows while we iterate and spawn.
        // It also makes "same event must not reprocess spawned matches" straightforward.
        self.buckets.remove(&state).unwrap_or_default()
    }

    /// Reinserts a processed bucket when it still has active paths.
    fn put_bucket(&mut self, state: StateId, bucket: Vec<ActiveMatch>) {
        if !bucket.is_empty() {
            self.buckets.insert(state, bucket);
        }
    }
}
