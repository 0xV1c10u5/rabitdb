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

### Roadmap
- Define Problem
- Efficient Save to Disk
- Basic Queryability
- ???
- Profit

#### AI Suggestion

📍 Phase 1: Foundations

Define Core Goals

What kind of database do you want first? (Key-value store, document store, relational, time series?)

What modular components should exist from day one (e.g., storage engine, query engine, indexing)?

Decide whether the extensibility will be through traits, plugins (dynamic libs), or feature flags.

Set Up Project Structure

Rust workspace with multiple crates (core, storage, query, network, cli, etc.).

Ensure each crate is independent and communicates via clearly defined interfaces/traits.

Core Abstractions

Define traits for key modules (e.g., StorageEngine, Transaction, Index, QueryExecutor).

Keep implementations separate from interfaces, so you can swap them later.

📍 Phase 2: Minimal Viable Database

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
