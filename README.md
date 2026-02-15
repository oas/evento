# evento

**evento** is an in-memory Complex Event Processing (CEP) engine built in Rust. It is designed to identify specific sequences and patterns within high-volume event streams using an efficient, state-machine-based approach.

The project is currently in **active development**. While the core execution engine and the Pattern API is functional, it is currently focused on single-node performance. The long-term goal is not only to provide a robust, low-latency alternative to heavy enterprise streaming frameworks, but also to introduce features like **dynamic pattern updates** and **state-aware matching logic**, enabling the engine to interpret the history of matched events and use complex logic to determine if a sequence remains valid.

---

The Pattern API is designed to be readable for business experts while remaining strictly typed for developers. You define patterns by chaining transitions:

```rust
...
```
