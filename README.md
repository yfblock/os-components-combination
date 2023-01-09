# os-components-combination

combine different components from different os.

## Step 1 Analysis & reuse driver

1. Analyze the [nvme_driver](https://github.com/rcore-os/nvme_driver) and [virt-io-blk](https://github.com/rcore-os/virtio-drivers/blob/master/src/device/blk.rs), two bare-metal executable blocks device driver.
2. Analyze how to use [nvme_driver](https://github.com/rcore-os/nvme_driver) in zCore and how to use [virt-io-blk](https://github.com/rcore-os/virtio-drivers/blob/master/src/device/blk.rs) in rCore-tutorial-v3.
3. Using [nvme_driver](https://github.com/rcore-os/nvme_driver) and [virt-io-blk](https://github.com/rcore-os/virtio-drivers/blob/master/src/device/blk.rs) in the os written by myself. Use [nvme_driver](https://github.com/rcore-os/nvme_driver) in rCore-Tutorail-v3.

## Step 2 Analysis & reuse filesystem

1. Analyze the easy-fs crate in rCore-Tutorial-v3
2. Analyze the rust-fatfs in my own os. Use it as independent fat32 crate, has the independent user mode test environment.(Don't need filesystem)
3. Using easy-fs crate and rust-fatfs crate in my own os and rCore-Tutorial-v3.