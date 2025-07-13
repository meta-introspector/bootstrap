# Bootstrap Stage0 - API Reference

## Overview

This document provides a comprehensive API reference for the Bootstrap Stage0 flow-based microkernel. Each type is a potential flow in a Navier-Stokes equation, with specific mathematical properties and operations.

## Core Types

### Hash Flow

**File**: `src/hash.rs`  
**Purpose**: Content identification through hash space

#### Struct Definition
```rust
pub struct Hash([u8; 32]);
```

#### Methods
```rust
impl Hash {
    /// Creates a new hash potential from flow field
    pub fn from_flow(data: &[u8]) -> Self
    
    /// Returns the flow field at this potential
    pub fn flow_field(&self) -> &[u8; 32]
    
    /// Computes the gradient of this hash potential
    pub fn gradient(&self) -> Vec<u8>
}
```

#### Flow Operator
```rust
/// Transforms content flow into hash potential
pub fn hash_flow(data: &[u8]) -> Hash
```

#### Mathematical Properties
- **Potential**: Content identification field
- **Gradient**: Rate of change in hash space
- **Flow Field**: 32-byte hash space
- **Convergence**: Content converges to unique hash

### Artifact Flow

**File**: `src/artifact.rs`  
**Purpose**: Content storage and materialization

#### Struct Definition
```rust
pub struct Artifact {
    pub hash: Hash,
    pub content: Vec<u8>,
}
```

#### Methods
```rust
impl Artifact {
    /// Creates a new artifact potential from content flow
    pub fn from_content_flow(content: Vec<u8>) -> Self
    
    /// Returns the content flow at this potential
    pub fn content_flow(&self) -> &[u8]
    
    /// Computes the divergence of this artifact potential
    pub fn divergence(&self) -> usize
}
```

#### Flow Operator
```rust
/// Transforms content into artifact potential
pub fn artifact_flow(content: Vec<u8>) -> Artifact
```

#### Mathematical Properties
- **Potential**: Content storage field
- **Divergence**: Content size/length
- **Materialization**: Content becomes concrete artifact
- **Flow**: Content flows through storage space

### Storage Flow

**File**: `src/storage.rs`  
**Purpose**: Storage field operations

#### Struct Definition
```rust
pub struct Storage {
    field: HashMap<Hash, Artifact>,
}
```

#### Methods
```rust
impl Storage {
    /// Creates a new storage potential field
    pub fn new_field() -> Self
    
    /// Stores an artifact in the storage field
    pub fn store_flow(&mut self, artifact: Artifact) -> Result<(), StorageFlowError>
    
    /// Retrieves an artifact from the storage field
    pub fn retrieve_flow(&self, hash: &Hash) -> Option<Artifact>
    
    /// Computes the curl of the storage field
    pub fn field_curl(&self) -> usize
}
```

#### Flow Operator
```rust
/// Transforms storage operations into storage potentials
pub fn storage_flow() -> Storage
```

#### Error Type
```rust
pub enum StorageFlowError {
    FlowFailed,
    NotFound,
}
```

#### Mathematical Properties
- **Field**: Storage potential field
- **Curl**: Number of stored artifacts
- **Mapping**: Hash to artifact mapping
- **Convergence**: Artifacts converge in storage field

### Kernel Flow

**File**: `src/kernel.rs`  
**Purpose**: System coordination and cycle management

#### Struct Definition
```rust
pub struct Kernel {
    storage: Storage,
    cycle: u64,
}
```

#### Methods
```rust
impl Kernel {
    /// Creates a new kernel potential field
    pub fn new_field() -> Self
    
    /// Stores content flow and returns hash potential
    pub fn store_flow(&mut self, content: Vec<u8>) -> Hash
    
    /// Retrieves artifact flow by hash potential
    pub fn retrieve_flow(&self, hash: &Hash) -> Option<Artifact>
    
    /// Advances the cycle flow (42-step cycle)
    pub fn advance_cycle_flow(&mut self)
    
    /// Gets the current cycle potential
    pub fn cycle_potential(&self) -> u64
    
    /// Computes the system flow divergence
    pub fn system_divergence(&self) -> usize
}
```

#### Flow Operator
```rust
/// Transforms system operations into kernel potentials
pub fn kernel_flow() -> Kernel
```

#### Mathematical Properties
- **Potential**: Central coordination field
- **Divergence**: System complexity (number of artifacts)
- **Cycle**: 42-step periodic evolution
- **Coordination**: All flows converge here

### System Flow

**File**: `src/system.rs`  
**Purpose**: Complete bootstrap system as unified flow field

#### Struct Definition
```rust
pub struct System {
    kernel: Kernel,
}
```

#### Methods
```rust
impl System {
    /// Creates a new system potential field
    pub fn new_field() -> Self
    
    /// Stores content in the system flow field
    pub fn store_system_flow(&mut self, content: Vec<u8>) -> Hash
    
    /// Retrieves content from the system flow field
    pub fn retrieve_system_flow(&self, hash: &Hash) -> Option<Artifact>
    
    /// Gets the current system cycle potential
    pub fn system_cycle_potential(&self) -> u64
    
    /// Computes the total system flow divergence
    pub fn total_system_divergence(&self) -> usize
    
    /// Computes the system flow curl (vorticity)
    pub fn system_curl(&self) -> u64
}
```

#### Flow Operators
```rust
/// Transforms system operations into unified system potentials
pub fn system_flow() -> System

/// The entry point into the complete flow field
pub fn bootstrap_flow() -> System
```

#### Mathematical Properties
- **Potential**: Higher-order unified field
- **Curl**: System vorticity (cycle state)
- **Divergence**: Total system complexity
- **Coordination**: All sub-flows unified here

## Main Interface

### Bootstrap Type

**File**: `src/lib.rs`  
**Purpose**: Main entry point into the flow system

#### Struct Definition
```rust
pub struct Bootstrap {
    system: System,
}
```

#### Methods
```rust
impl Bootstrap {
    /// Creates a new bootstrap flow field
    pub fn new() -> Self
    
    /// Stores content in the bootstrap flow
    pub fn store(&mut self, content: Vec<u8>) -> Hash
    
    /// Retrieves content from the bootstrap flow
    pub fn retrieve(&self, hash: &Hash) -> Option<Artifact>
    
    /// Gets the current cycle potential
    pub fn cycle_step(&self) -> u64
    
    /// Computes the total system divergence
    pub fn total_divergence(&self) -> usize
    
    /// Computes the system curl (vorticity)
    pub fn curl(&self) -> u64
}
```

#### Default Implementation
```rust
impl Default for Bootstrap {
    fn default() -> Self
}
```

## Flow Operations Summary

### Hash Operations
- `hash_flow(data) -> Hash`: Transform content to hash potential
- `Hash::from_flow(data) -> Hash`: Create hash from content
- `hash.gradient() -> Vec<u8>`: Compute hash gradient
- `hash.flow_field() -> &[u8; 32]`: Get hash flow field

### Artifact Operations
- `artifact_flow(content) -> Artifact`: Transform content to artifact
- `Artifact::from_content_flow(content) -> Artifact`: Create artifact from content
- `artifact.divergence() -> usize`: Compute artifact divergence
- `artifact.content_flow() -> &[u8]`: Get content flow

### Storage Operations
- `storage_flow() -> Storage`: Create storage field
- `storage.store_flow(artifact) -> Result<(), StorageFlowError>`: Store artifact
- `storage.retrieve_flow(hash) -> Option<Artifact>`: Retrieve artifact
- `storage.field_curl() -> usize`: Compute field curl

### Kernel Operations
- `kernel_flow() -> Kernel`: Create kernel field
- `kernel.store_flow(content) -> Hash`: Store content through kernel
- `kernel.retrieve_flow(hash) -> Option<Artifact>`: Retrieve through kernel
- `kernel.cycle_potential() -> u64`: Get cycle state
- `kernel.system_divergence() -> usize`: Get system complexity

### System Operations
- `system_flow() -> System`: Create system field
- `bootstrap_flow() -> System`: Create bootstrap system
- `system.store_system_flow(content) -> Hash`: Store in system
- `system.retrieve_system_flow(hash) -> Option<Artifact>`: Retrieve from system
- `system.system_curl() -> u64`: Get system vorticity
- `system.total_system_divergence() -> usize`: Get total complexity

## Error Handling

### StorageFlowError
```rust
pub enum StorageFlowError {
    #[error("Storage flow operation failed")]
    FlowFailed,
    #[error("Hash potential not found in field")]
    NotFound,
}
```

## Mathematical Properties Reference

### Flow Properties
- **Gradient**: Rate of change in potential (Hash)
- **Divergence**: Flow source/sink strength (Artifact, Kernel, System)
- **Curl**: Vorticity of the flow field (Storage, System)
- **Potential**: Scalar field from which flow derives (all types)

### 42-Step Cycle
- **Period**: 42 operations before cycle completion
- **Advancement**: Each storage operation advances cycle
- **Wrapping**: Cycle wraps around at 42 (modulo 42)
- **State**: Current cycle state represents system evolution

## Usage Patterns

### Basic Flow
1. Create bootstrap system: `Bootstrap::new()`
2. Store content: `bootstrap.store(content)`
3. Retrieve content: `bootstrap.retrieve(&hash)`
4. Check properties: `bootstrap.cycle_step()`, `bootstrap.curl()`

### Advanced Flow Composition
1. Create individual flows: `hash_flow()`, `artifact_flow()`, etc.
2. Compose flows: Chain operations through different flow types
3. Analyze properties: Check gradients, divergences, curls
4. Monitor evolution: Track cycle advancement and system state

### Error Handling
```rust
match storage.store_flow(artifact) {
    Ok(()) => println!("Flow stored successfully"),
    Err(StorageFlowError::FlowFailed) => println!("Flow operation failed"),
    Err(StorageFlowError::NotFound) => println!("Hash not found in field"),
}
``` 