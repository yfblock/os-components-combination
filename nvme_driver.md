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