# rabitdb
Attempt at a Modular Database

## Description
In my limited time, I wanted to develop a database for fun. The timeline for this project is very long, and I intend to work on it in sections. I wanted to clarify intent so no one uses this in production. Please do not deploy this on any critical infrastructure, especially before any 1.0 version is released.

### Project Goals
(These may be unrealistic depending on challenges faced.)

- Mimimal/Non-existent Dependencies (Build core functionalities, understanding the technical debt this adds)
- One Program, Multiple Databases
- Reduce Friction between sharing data between languages (Through Standardization/Wrapper Libraries)
- SQL Backwards Compatibility
- Modularity (Add Data Layers as needed, interact with one library)
- Distributed if possible

### Conceptualization
- Achieve Modularity through Layers: Each new "Database" is a layer of interaction. When the data is created, it's automatically assigned a layer of interaction.  To clarify further, instead of setting up a manual connection to the DB, instead the connection is established through a config/library similar to IaC. If there is no config file, the library looks for a local application running the db. (If neither are found, return error, or create local datastore)

### **Notes:**

- Might need to separate DB core from library, implement query inside library not core(?)

### Roadmap
- Define Problem
- B+ Tree
- Efficient Save to Disk
- Basic Queryability
- C Bindings
- Property-Based Testing
- Stress Testing
- Edge-Testing
- Integration Testing
- E2E Testing
- Regression Testing
- Benchmark Testing
- Serialization Testing
- Combinatorial Testing
- Integrity Testing
- Reliability Testing
- Transactional Testing
- ???
- Profit

### Guardrails
- add core rust testing (cargo test, cargo fmt, cargo fuzz, cargo clippy, cargo miri test, etc) to CI/CD
- sanitizers: If you enable -Zsanitizer=address (nightly Rust), you can run with AddressSanitizer to catch memory corruption bugs.
- observability: log crate, env_logger crate, tracing crate
- use '///' documentation notes for core testing when possible:
> /// Adds two numbers together.
/// 
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// assert_eq!(add(2, 3), 5);
/// ```
- Incremental delivery & feature flags

Instead of shipping a huge batch of changes at once, ship smaller, isolated features — often behind feature flags.

Rust has Cargo features (for conditional compilation).

You can also implement runtime feature flags (via config/env vars).

Cargo feature example (Cargo.toml):

[features]
experimental = []


Code:

#[cfg(feature = "experimental")]
pub fn new_algorithm() {
    // some risky new feature
}


Run with:

cargo run --features experimental
- Specification & property-driven design

This is about thinking in invariants (rules that must always hold true), and then baking them into your code/tests.

Specification: Write down the rules.

Example: A bank account balance must never go negative.

Property-driven design: Encode rules in types and property tests.

Type-system enforcement:

struct Balance(u32); // cannot be negative by design


Property-based test with proptest:

use proptest::prelude::*;

proptest! {
    #[test]
    fn balance_never_negative(a in 0u32..1000, b in 0u32..1000) {
        let result = a.checked_sub(b);
        // property: subtraction must never produce a negative balance
        prop_assert!(result.is_some() || a < b);
    }

- contract tests: API
- chaos testing: disaster recovery
- code coverage tracking: cargo-llvm-cov
- sec commands: cargo audit, cargo deny

### Function Namespace + Taxonomy
- core
- storage
- query
- network
- cli
- indexing

#### Technicals

Basic Storage Layer
Implement a simple log-structured storage engine (append-only file or memory-mapped file).
Add a basic key-value API (put, get, delete).
Indexing
Start with in-memory structures (like a BTreeMap).
Later, abstract it behind an Index trait to allow on-disk indexes.
Query Layer
Implement a minimal query executor that works on top of the storage engine.
For a KV store: simple gets/puts; for a document/relational style: basic filtering.

📍 Phase 3: Extensibility & Modularity
Trait-based Interfaces
Abstract over storage engines (MemoryEngine, FileEngine, LMDB/RocksDB wrapper).
Pluggable query executors or indexes.
Plugin System
Decide if extensions are dynamic (shared libraries loaded at runtime) or compile-time features.
Define an extension API (Rust traits or even WASM modules for safety).
Configuration System
Make modules configurable via a config file or builder pattern.
Allow the user to choose storage engine, indexing method, etc.

📍 Phase 4: Advanced Features
Transactions
Add a WAL (Write Ahead Log).
Implement basic ACID guarantees (start with atomicity + durability).
Concurrency & Safety
Explore Arc<Mutex<T>>, RwLock, or lock-free data structures.
Implement multi-threaded query execution.
Networking
Add a simple TCP/HTTP protocol for client-server communication.
Define a wire protocol (JSON, protobuf, or custom).
Query Language
Start with a simple DSL or JSON-based query structure.
Later, consider SQL-like parsing (using a Rust parser library like nom).

📍 Phase 5: Testing, Benchmarking & Tooling
Testing
Property-based tests (e.g., with proptest).
Fuzzing (Rust has good fuzzing support).
Test modular swaps (e.g., swapping storage engines doesn’t break queries).
Benchmarking
Compare different storage/index modules with microbenchmarks.
Implement simple performance monitoring hooks.
Developer Tools
Command-line interface (REPL or simple client).
Admin tools for inspecting storage files.

📍 Phase 6: Long-Term Growth
Support clustering/distributed setup (later stage).
Add security modules (authentication, encryption).
Support schema evolution/migrations.
Document an API for third-party developers to add new modules.

⚡ Tips & Gotchas

Rust’s ownership model will save you from many bugs but will complicate abstractions—plan traits carefully.
Don’t over-engineer early—build a minimal working system before adding modularity.
Study existing Rust DBs:
sled (embedded KV store, modular, log-structured).
tikv (distributed KV with Rust core).
databend (cloud warehouse, modular).
Use workspaces heavily to separate crates cleanly.

#### AI Suggestion

What modular components should exist from day one (e.g., storage engine, query engine, indexing)?

**Decide Whether the extensibility will be through traits, plugins (dynamic libs), or feature flags.**

Rust workspace with multiple crates (core, storage, query, network, cli, etc.).

Ensure each crate is independent and communicates via clearly defined interfaces/traits.

Core Abstractions

Define traits for key modules (e.g., StorageEngine, Transaction, Index, QueryExecutor).

**Keep implementations separate from interfaces, so you can swap them later.**
# Arc(itecture)
Onboarding/Design Explanation

## What is this document?
It seems very quickly it might make sense to have a journal of design decisions and quick references to explanations of relevant concepts, in order to help anyone who might want to contribute, as well as myself in the future if there is a lapse in development time for whatever reason.

### Core Concepts

What is a "B+ Tree"? Why is it important to what we are trying to build?
<p>
🌳 What is a B+ Tree?

A B+ tree is a balanced multiway search tree optimized for block-oriented storage (disk pages, SSD pages, memory cache lines). It’s the standard index structure in databases.

Key traits:

Internal nodes: only store keys and child pointers (no values).

Leaf nodes: store all the (key, value) pairs.

Linked leaves: leaves are connected in a linked list → fast range scans.

Balanced: all leaves are at the same depth.

High fanout: each node holds many keys (hundreds or thousands if node size ≈ 4 KB).

🔑 Why Databases Use B+ Trees

Disk/SSD efficiency: One node ≈ one page. Accessing a key = just a few page reads.

Shallow depth: With fanout ≈ hundreds, even a billion rows fit in 3–4 levels.

Range queries: Linked leaves make BETWEEN and prefix lookups very fast.

Good concurrency: Locks at node/page level are manageable.

Updates are localized: Only a few nodes split/merge per insert/delete.

That’s why almost every RDBMS (Postgres, MySQL, SQLite, etc.) uses B+ trees as their main index structure.

🦀 Implementing a Simple B+ Tree in Rust

Here’s a toy in-memory implementation with:

Internal nodes: guide search

Leaves: store key/value pairs + next pointer

Basic insert + search

This is educational (not production-grade). Real DB implementations manage pages, caching, and concurrency.

use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

const ORDER: usize = 4; // max children per internal node

// Node type: Internal or Leaf
enum Node<K, V> {
    Internal {
        keys: Vec<K>,
        children: Vec<Rc<RefCell<Node<K, V>>>>,
    },
    Leaf {
        keys: Vec<K>,
        values: Vec<V>,
        next: Option<Rc<RefCell<Node<K, V>>>>,
    },
}

pub struct BPlusTree<K, V> {
    root: Rc<RefCell<Node<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> BPlusTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::Leaf {
                keys: Vec::new(),
                values: Vec::new(),
                next: None,
            })),
        }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        let mut node = self.root.clone();
        loop {
            match &*node.borrow() {
                Node::Internal { keys, children } => {
                    // binary search
                    let mut idx = keys.binary_search(key).unwrap_or_else(|i| i);
                    if idx >= children.len() { idx = children.len() - 1; }
                    node = children[idx].clone();
                }
                Node::Leaf { keys, values, .. } => {
                    return keys.iter()
                        .position(|k| k == key)
                        .map(|i| values[i].clone());
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let root = self.root.clone();
        let (new_key, new_node) = Self::insert_rec(root.clone(), key, value);

        if let Some((k, child)) = new_key.zip(new_node) {
            // root split
            let new_root = Node::Internal {
                keys: vec![k],
                children: vec![root, child],
            };
            self.root = Rc::new(RefCell::new(new_root));
        }
    }

    fn insert_rec(
        node: Rc<RefCell<Node<K, V>>>,
        key: K,
        value: V,
    ) -> (Option<K>, Option<Rc<RefCell<Node<K, V>>>>) {
        let mut n = node.borrow_mut();

        match &mut *n {
            Node::Leaf { keys, values, next } => {
                match keys.binary_search(&key) {
                    Ok(i) => values[i] = value, // overwrite
                    Err(i) => {
                        keys.insert(i, key);
                        values.insert(i, value);
                    }
                }
                if keys.len() >= ORDER {
                    // split leaf
                    let mid = keys.len() / 2;
                    let split_keys = keys.split_off(mid);
                    let split_vals = values.split_off(mid);

                    let new_leaf = Rc::new(RefCell::new(Node::Leaf {
                        keys: split_keys.clone(),
                        values: split_vals,
                        next: next.take(),
                    }));

                    *next = Some(new_leaf.clone());
                    let sep_key = split_keys[0].clone();
                    return (Some(sep_key), Some(new_leaf));
                }
                (None, None)
            }

            Node::Internal { keys, children } => {
                let mut idx = keys.binary_search(&key).unwrap_or_else(|i| i);
                let (new_key, new_child) = Self::insert_rec(children[idx].clone(), key, value);

                if let Some((k, child)) = new_key.zip(new_child) {
                    keys.insert(idx, k);
                    children.insert(idx + 1, child);

                    if keys.len() >= ORDER {
                        // split internal node
                        let mid = keys.len() / 2;
                        let sep_key = keys[mid].clone();

                        let right_keys = keys.split_off(mid + 1);
                        let right_children = children.split_off(mid + 1);

                        let new_internal = Rc::new(RefCell::new(Node::Internal {
                            keys: right_keys,
                            children: right_children,
                        }));

                        keys.pop(); // remove sep_key from left half
                        return (Some(sep_key), Some(new_internal));
                    }
                }
                (None, None)
            }
        }
    }
}

// --- demo ---
fn main() {
    let mut tree = BPlusTree::new();
    for (k, v) in [(1,"a"),(2,"b"),(3,"c"),(4,"d"),(5,"e"),(6,"f")] {
        tree.insert(k, v);
    }

    println!("{:?}", tree.search(&3)); // Some("c")
    println!("{:?}", tree.search(&10)); // None
}

🚀 Next Steps Toward Database-Grade B+ Trees

Page-based storage: Nodes should be serialized into fixed-size pages (e.g., 4 KB).

Buffer manager: Cache hot pages in memory, flush dirty ones.

Persistence: Replace Rc<RefCell<_>> with page IDs + disk-backed pager.

Range scans: Add an iterator that follows leaf.next.

Concurrency: Implement latch crabbing (lock-coupling).

Deletes & merges: Balance underflowing nodes by borrowing/merging.

### Desired Features
