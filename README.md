# evento

**evento** is a Complex Event Processing (CEP) engine built in Rust. It is designed to identify specific sequences and patterns within high-volume event streams using an efficient, state-machine-based approach.

The project is currently in **active development**. While the core execution engine and the Pattern API is functional, it is currently focused on single-node performance. The long-term goal is not only to provide a robust, low-latency alternative to heavy enterprise streaming frameworks, but also to introduce features like **dynamic pattern updates** and **state-aware matching logic**, enabling the engine to interpret the history of matched events and use complex logic to determine if a sequence remains valid.

---

The Pattern API is designed to be readable for business experts while remaining strictly typed for developers. You define patterns by chaining transitions:

```rust
let pattern = PatternBuilder::new(1)
	// 1. start with a successful login
	.then("login", Some("login"), Arc::new(|ev, _ctx| {
		match ev.payload.get("user_id") {
			Some(EventValue::Int(id)) => *id > 0,
			_ => false, // No user_id or wrong type? No match.
		}
	}))
	
	// 2. look for either a large purchase OR a large transfer
	.either(vec![
		StepDefinition::new().then("large_purchase", Some("purchase"), Arc::new(|ev, _| {
			match ev.payload.get("amount") {
				Some(EventValue::Float(a)) => *a >= 100.0,
				_ => false,
			}
		})),
		StepDefinition::new().then("large_transfer", Some("transfer"), Arc::new(|ev, _| {
			match ev.payload.get("amount") {
				Some(EventValue::Float(a)) => *a >= 100.0,
				_ => false,
			}
		})),
	])
	// (this whole 'either' block must complete within 30 seconds of the login)
	.within(30_000)

	// 3. finally, compile the pattern into a nondeterministic finite automaton (NFA) for execution
	.compile();
```

