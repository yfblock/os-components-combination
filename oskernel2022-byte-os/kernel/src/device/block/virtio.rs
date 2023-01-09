use crate::device::BLK_CONTROL;

use super::super::BlockDevice;
use alloc::boxed::Box;
use arch::VIRTIO0;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

pub struct VirtIOBlock(pub VirtIOBlk<'static>);

impl BlockDevice for VirtIOBlock {
    fn read_block(&mut self, sector_offset: usize, buf: &mut [u8]) {
        self.0.read_block(sector_offset, buf).expect("读取失败")
    }

    fn write_block(&mut self, sector_offset: usize, buf: &mut [u8]) {
        self.0.read_block(sector_offset, buf).expect("写入失败")
    }

    fn handle_irq(&mut self) {
        todo!()
    }
}

pub fn init() {
    // 创建存储设备
    let device = Box::new(VirtIOBlock(
        VirtIOBlk::new(unsafe {&mut *(VIRTIO0 as *mut VirtIOHeader)}).expect("failed to create blk driver")
    ));
    // 加入设备表
    unsafe {
        BLK_CONTROL.push(device)
    };
}