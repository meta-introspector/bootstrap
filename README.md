# Bootstrap - Minimal Self-Contained Microkernel

A truly minimal, dependency-free bootstrap system that provides the essential building blocks for content-addressable storage and dynamic extension.

## Core Principles

- **Zero Dependencies**: No external crates, pure Rust std/core
- **Self-Contained**: Complete functionality within this crate
- **Content-Addressable**: All data identified by content hash
- **Extensible**: Pluggable components via traits
- **Minimal**: Only essential functionality for bootstrap

## Quick Start

```rust
use bootstrap::{Bootstrap, Artifact, Hash};

// Create a new bootstrap system
let mut bootstrap = Bootstrap::new();

// Store content
let content = b"Hello, World!".to_vec();
let hash = bootstrap.store(content.clone());

// Retrieve content
let retrieved = bootstrap.retrieve(&hash);
assert!(retrieved.is_some());
assert_eq!(retrieved.unwrap().content, content);

// Check cycle step
println!("Current step: {}", bootstrap.cycle_step()); // 0-41
```

## Architecture

### Core Components

- **Artifact**: Content-addressable data units
- **Hash**: Content identification via hashing
- **Kernel**: System orchestration with 42-step cycle
- **Storage**: Pluggable storage backends

### Key Features

- **42-Step Cycle**: System advances through a cycle of operations
- **Content Deduplication**: Identical content produces identical hashes
- **Component Replacement**: Hash and storage components can be swapped
- **Memory Storage**: Simple in-memory storage for bootstrap

## Usage

### Basic Storage

```rust
let mut bootstrap = Bootstrap::new();

// Store different content
let hash1 = bootstrap.store(b"content1".to_vec());
let hash2 = bootstrap.store(b"content2".to_vec());

// Same content produces same hash
let hash3 = bootstrap.store(b"content1".to_vec());
assert_eq!(hash1, hash3);
```

### Component Replacement

```rust
use bootstrap::{SimpleHasher, ChecksumHasher, MemoryStore};

let mut bootstrap = Bootstrap::new();

// Replace hasher (advances cycle)
bootstrap.kernel.replace_hasher(Box::new(ChecksumHasher));

// Replace storage (advances cycle)
bootstrap.kernel.replace_store(Box::new(MemoryStore::new()));
```

## Development

### Building

```bash
cd bootstrap
cargo build
```

### Testing

```bash
cargo test
```

### Adding Components

1. Implement the appropriate trait (`Hasher`, `Store`)
2. Add to the kernel via replacement methods
3. The cycle advances automatically

## Design Decisions

### Why Zero Dependencies?

- **Bootstrap Capability**: Can run in minimal environments
- **Security**: No external attack vectors
- **Control**: Complete control over all functionality
- **Simplicity**: Easy to understand and audit

### Why 42 Steps?

- **Finite Cycle**: Manageable number for tracking
- **Self-Reference**: System consumes and renews itself
- **Cosmic Order**: Reference to "The Hitchhiker's Guide to the Galaxy"

### Why Content-Addressable?

- **Deduplication**: Identical content stored once
- **Integrity**: Content cannot be modified without new hash
- **Immutability**: Content references are stable

## Future Extensions

This minimal bootstrap provides the foundation for:

- **File Storage**: Persistent storage backends
- **Network Storage**: Distributed storage systems
- **Advanced Hashing**: Cryptographic hash functions
- **Plugin System**: Dynamic component loading
- **Distributed Coordination**: Multi-node systems

## License

MIT License - see LICENSE file for details. 