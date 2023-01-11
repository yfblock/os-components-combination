# rust-fatfs driver usage record

## ported with rCore-Tutorial-v3

### Analysis easy-fs

> Before we port rust-fatfs, we should know what we need to do.

```rust
//!An easy file system isolated from the kernel
#![no_std]
#![deny(missing_docs)]
extern crate alloc;
mod bitmap;
mod block_cache;
mod block_dev;
mod efs;
mod layout;
mod vfs;
/// Use a block size of 512 bytes
pub const BLOCK_SZ: usize = 512;
use bitmap::Bitmap;
use block_cache::{block_cache_sync_all, get_block_cache};
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
use layout::*;
pub use vfs::Inode;
```

We can see that mod export `BLOCK_SZ`, `vfs::Inode`, `EasyFileSystem` and `BlockDevice`. So we can use the same interface in `rust-fatfs` instead.

### create crate easy-fatfs

```
cargo new easy-fatfs
```

Copy the file in the `easy-fs` to this folder.

### Anything different

> To make the porting process easier, we're going to use a better method. Only the `Cargo.toml` file needs to ne modified and not the source code.

Firstly, add `cargo-features = ["rename-dependency"]` at the top of the `Cargo.toml` file.

And then, we can rename dependency in the `Cargo.toml` file.

eg:
```toml
easy-fs = { path = "../easy-fatfs", package = "easy-fatfs" }
```

rename `easy-fatfs` to `easy-fs`.

### add rust-fatfs to cargo.toml

```toml
fatfs = { version = "0.4.0", default-features = false, features = ["alloc", "lfn", "log_level_trace", "unicode"] }

[patch.crates-io]
fatfs = { git = "https://github.com/rafalh/rust-fatfs" }
```

### Analyze easy-fs

The following are the minimum requirements for successful compilation.

> efs.rs

```rust
use crate::{BLOCK_SZ, BlockDevice, Inode};
use alloc::sync::Arc;
use spin::Mutex;
///An easy file system on block
pub struct EasyFileSystem {}

type DataBlock = [u8; BLOCK_SZ];
/// An easy fs over a block device
impl EasyFileSystem {
    /// Open a block device as a filesystem
    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
        todo!()
    }
    /// Get the root inode of the filesystem
    pub fn root_inode(efs: &Arc<Mutex<Self>>) -> Inode {
        todo!()
    }
}
```

> vfs.rs

```rust
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};

use crate::{EasyFileSystem, BlockDevice};
/// Virtual filesystem layer over easy-fs
pub struct Inode {
    block_id: usize,
    block_offset: usize,
    fs: Arc<Mutex<EasyFileSystem>>,
    block_device: Arc<dyn BlockDevice>,
}

impl Inode {
    /// Find inode under current inode by name
    pub fn find(&self, name: &str) -> Option<Arc<Inode>> {
        todo!()
    }
    /// Create inode under current inode by name
    pub fn create(&self, name: &str) -> Option<Arc<Inode>> {
        todo!()
        // release efs lock automatically by compiler
    }
    /// List inodes under current inode
    pub fn ls(&self) -> Vec<String> {
        todo!()
    }
    /// Read data from current inode
    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> usize {
        todo!()
    }
    /// Write data to current inode
    pub fn write_at(&self, offset: usize, buf: &[u8]) -> usize {
        todo!()
    }
    /// Clear the data in current inode
    pub fn clear(&self) {
        todo!()
    }
}

```

### Finally (to be continued)

> pass