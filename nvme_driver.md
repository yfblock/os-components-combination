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

1. I try to see the source code of the example.