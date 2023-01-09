# Nvme driver usage record

## environment

> basic
- __platform:__ qemu-riscv64
- __host os:__ Manjaro 22.0.0
- __os:__ byteos

> other
- __platform:__ qemu-riscv64
- __host os:__ Manjaro 22.0.0
- __os:__ rcore-tutorial-v3

## introduction

repo: [https://github.com/rcore-os/nvme_driver](https://github.com/rcore-os/nvme_driver)

local test folder: 
- nvme_driver

## try example

1. Download source code locally, unzip it to nvme_driver folder
2. Run example
    ```shell
    cd nvme_driver/example
    dd if=/dev/zero bs=1M count=128 of=nvme.img
    make qemu-nvme
    ```
3. See the change of nvme.img file
    ```shell
    cat | head -c 1024 nvme.img | xxd
    ```

## analyze

### I/O mode
> I try to look at the source code of the example. It has some unused code. In order to find the driver relationship, I delete the unused code.

the new src folder tree is:
```plain
src
├── console.rs
├── entry.asm
├── lang_items.rs
├── linker64.ld
├── main.rs
├── nvme.rs
└── sbi.rs
```

The most important file is nvme.rs. Contains the main code related to nvme driver.

In order to use nvme driver, pci should be configured first.

```rust
use core::ptr::write_volatile;

// config pci
pub fn config_pci(){
    let ptr = 0x30008010 as *mut u32;
    unsafe { write_volatile(ptr, 0xffffffff); }
    let ptr = 0x30008010 as *mut u32;
    unsafe { write_volatile(ptr, 0x4); }
    let ptr = 0x30008010 as *mut u32;
    unsafe { write_volatile(ptr, 0x40000000); }
    let ptr = 0x30008004 as *mut u32;
    unsafe { write_volatile(ptr, 0x100006); }
    let ptr = 0x3000803c as *mut u32;
    unsafe { write_volatile(ptr, 0x21); }
}
```

And then, define a struct and implement `DmaAllocator` trait for it.

```rust
pub struct DmaProvider;

impl DmaAllocator for DmaProvider{

    fn dma_alloc(size: usize) -> usize{
        let paddr = DMA_PADDR.fetch_add(size, Ordering::SeqCst);
        paddr
    }

    fn dma_dealloc(addr: usize, size: usize) -> usize{
        0
    }

    fn phys_to_virt(phys: usize) -> usize{
        phys
    }

    fn virt_to_phys(virt: usize) -> usize{
        virt
    }
}
```

Define a struct and implement `IrqController` trait for it.

```rust
pub struct IrqProvider;

impl IrqController for IrqProvider{
    fn enable_irq(irq: usize){
        
    }

    fn disable_irq(irq: usize){
        
    }
}
```

After that, I can use `NvmeInterface` to define the variable and use it to read or write memory blocks.


```rust
let nvme = NvmeInterface::<DmaProvider, IrqProvider>::new(0x40000000);
// read block
let mut read_buf = [0u8; 512];
nvme.read_block(i, &mut read_buf);
// write block
let write_buf:&[u8] = &[1u8;512];
nvme.write_block(i, &write_buf);
```

> Easy end! 🎉🎉🎉

## try to use it in bare-metal os

> get bare-metal from [try_async_bare_metal_os](https://github.com/yfblock/try_async_bare_metal_os)

__local folder:__ bare_metal

### step 1 Add dependency to cargo.toml

```toml
nvme_driver = { path = "../nvme_driver" }
```

### step 2 Use nvme instead of virtio


Crate a `block` folder and add `virtio.rs` and `nvme.rs`.

Create a `BlockDevice` trait as a compatibility layer.

```rust
pub trait BlockDevice {
    // read_block
    fn read_block(&mut self, sector_offset: usize, buf: &mut [u8]);
    // write_block
    fn write_block(&mut self, sector_offset: usize, buf: &[u8]);
    // handle_irq
    fn handle_irq(&mut self);
}
```

Define a global variable in `block/mod.rs`

```rust
pub static mut DEVICE: Once<Mutex<Box<dyn BlockDevice>>> = Once::new();
```

And define the `struct` and `init` function in each file.

> nvme.rs

```rust
...

// 虚拟IO设备
pub struct VirtIOBlock(pub NvmeInterface::<DmaAllocatorImpl, IrqControllerImpl>);

impl BlockDevice for VirtIOBlock {
    fn read_block(&mut self, sector_offset: usize, buf: &mut [u8]) {
        // 读取文件
        self.0.read_block(sector_offset, buf)
    }

    fn write_block(&mut self, sector_offset: usize, buf: &[u8]) {
        self.0.write_block(sector_offset, buf)
    }

    fn handle_irq(&mut self) {
        todo!()
    }
}

...
pub fn init() {
    // 初始化 pci
    config_pci();

    unsafe {
        // 创建存储设备
        DEVICE.call_once(|| {
            let device = Box::new(VirtIOBlock(
                NvmeInterface::<DmaAllocatorImpl, IrqControllerImpl>::new(0x40000000)
            ));
            Mutex::new(device)
        });
    }
}
```

> virtio.rs

```rust
...

pub struct VirtIOBlock(pub VirtIOBlk::<HalImpl, MmioTransport>);

impl BlockDevice for VirtIOBlock {
    fn read_block(&mut self, sector_offset: usize, buf: &mut [u8]) {
        self.0.read_block(sector_offset, buf);
    }

    fn write_block(&mut self, sector_offset: usize, buf: &[u8]) {
        self.0.write_block(sector_offset, buf);
    }

    fn handle_irq(&mut self) {
        todo!()
    }
}

pub fn init() {
    unsafe {
        DEVICE.call_once(|| {
            let header = NonNull::new(0x10001000 as *mut VirtIOHeader).unwrap();
            let transport = unsafe { MmioTransport::new(header) }.unwrap();
            let device = VirtIOBlk::<HalImpl, MmioTransport>::new(transport)
                .expect("failed to create blk driver");
            Mutex::new(Box::new(VirtIOBlock(device)))
        });
    }
}
```

And then you can select the driver which will be used in `mod.rs` file.

```rust
mod virtio;
mod nvme;

use alloc::boxed::Box;
use spin::Once;

// use virtio as block driver
// pub use virtio::init;

// use nvme as block driver
pub use nvme::init;

pub static mut DEVICE: Once<Mutex<Box<dyn BlockDevice>>> = Once::new();
```

### step 3 Add run command to makefile

```shell
run-nvme: all
	qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -drive file=fat32.img,if=none,id=nvm \
    -device nvme,serial=deadbeef,drive=nvm \
    -kernel kernel-qemu \
    -nographic \
    -smp 4 -m 2G
```

### step 4 Copy fat32.img file from oskernel2022-byte-os

```shell
    cp ../oskernel2022-byte-os/fs-origin.img fat32.img
```

### step 5 run test

```plain
程序大小为: 220 kb  堆大小: 128 kb  代码段: 60 kb
Hello nvme
async number: 42
async number: 42
Hello WOrld!
```

### step 6 then try to list files

```rust
let fs = fatfs::FileSystem::new(c, fatfs::FsOptions::new()).expect("open fs failed");
let mut cursor =fs.root_dir();

for file in cursor.iter() {
    if let Ok(file) = file {
        println!("{:>2$}├──{}", "", file.file_name(), 0);
    }
}
```

**but get some error**, the data between two reads is different. After communicating with author, we find the solution.
**It should use a new buffer every time it reads.**

### step 7 Run new Test

```plain
程序大小为: 216 kb  堆大小: 128 kb  代码段: 52 kb
├──busybox
├──busybox_cmd.txt
├──busybox_testcode.sh
├──date.lua
├──file_io.lua
├──lmbench_all
├──lmbench_testcode.sh
├──lua
├──lua_testcode.sh
├──max_min.lua
├──random.lua
├──remove.lua
├──round_num.lua
├──sin30.lua
├──sort.lua
├──strings.lua
├──test.sh
├──var
├──byte-test.sh
async number: 42
async number: 42
Hello WOrld!
rm nvme.img
```

## try to use it in byteos

Follow the logic above. If I want to use nvme with byteos. I just need to adjust the initialization code and change the read and write code from virtio-blk to nvme.

### step 1 Download byteos locally

__folder:__ oskernel2022-byte-os

### step 2 Test 

```shell
cd oskernel2022-byte-os
make run
```

### step 3 Run with nvme image

add run command to Makefile
```shell
    run-nvme: qemu
	@cp fs-origin.img nvme.img
	@qemu-system-riscv64 \
            -machine virt \
            -bios $(BOOTLOADER) \
            -device loader,file=$(BIN_FILE),addr=0x80200000 \
			-drive file=nvme.img,if=none,id=nvm \
			-device nvme,serial=deadbeef,drive=nvm \
			-kernel $(BIN_FILE) \
			-nographic \
			-smp 4 -m 128m
	@rm nvme.img    
```

### step 4 Modify source code to adapt nvme

> add dependency to Cargo.toml

```toml

```