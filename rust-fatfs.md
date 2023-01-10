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