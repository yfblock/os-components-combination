use alloc::boxed::Box;
// use nvme_driver::
use nvme_driver::{DmaAllocator, IrqController, NvmeInterface};
use virtio_drivers::VirtIOBlk;
use crate::device::{BlockDevice, BLK_CONTROL};
use crate::memory::page::PAGE_ALLOCATOR;
use crate::memory::addr::PhysPageNum;

pub struct DmaAllocatorImpl;
impl DmaAllocator for DmaAllocatorImpl {
    fn dma_alloc(size: usize) -> usize{
        // 申请内存
        println!("alloc memeory: {}", size);
        PAGE_ALLOCATOR.lock().alloc_more(size / 0x1000).expect("alloc error").into()
    }

    fn dma_dealloc(addr: usize, size: usize) -> usize{
        PAGE_ALLOCATOR.lock().dealloc_more(PhysPageNum::from(addr), size / 0x1000);
        0
    }

    fn phys_to_virt(phys: usize) -> usize{
        phys
    }

    fn virt_to_phys(virt: usize) -> usize{
        virt
    }
}

pub struct IrqControllerImpl;

impl IrqController for IrqControllerImpl {
    fn enable_irq(_irq: usize){

    }

    fn disable_irq(_irq: usize){

    }
}


// 虚拟IO设备
pub struct VirtIOBlock(pub NvmeInterface::<DmaAllocatorImpl, IrqControllerImpl>);

impl BlockDevice for VirtIOBlock {
    fn read_block(&mut self, sector_offset: usize, buf: &mut [u8]) {
        // 读取文件
        self.0.read_block(sector_offset, buf)
    }

    fn write_block(&mut self, sector_offset: usize, buf: &mut [u8]) {
        self.0.read_block(sector_offset, buf)
    }

    fn handle_irq(&mut self) {
        todo!()
    }
}

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
    info!("nvme pci 配置完毕");
}

pub fn init() {
    info!("初始化nvme pci");
    // 初始化 pci
    // config_pci();

    // 创建存储设备
    let device = Box::new(VirtIOBlock(
        NvmeInterface::<DmaAllocatorImpl, IrqControllerImpl>::new(0x40000000)
    ));
    info!("nvme 初始化完毕");
    // 加入设备表
    unsafe {
        BLK_CONTROL.push(device)
    };
    info!("nvme 初始化完毕");
}