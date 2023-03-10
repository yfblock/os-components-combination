use kernel::{task::fd_table::IoVec, memory::addr::UserAddr, runtime_err::RuntimeError, fs::file::FileOP};

use crate::SyscallTask;

// 读取
pub fn sys_read(task: SyscallTask, fd: usize, buf_ptr: UserAddr<u8>, count: usize) -> Result<(), RuntimeError> {
    // debug!("sys_read, fd: {}, buf_ptr: {:#x}, count: {}", fd, buf_ptr.bits(), count);
    let buf = buf_ptr.transfer_vec(count);
    let mut inner = task.inner.borrow_mut();
    let mut process = inner.process.borrow_mut();

    // 判断文件描述符是否存在
    let reader = process.fd_table.get(fd)?;
    let value = if reader.readable() {
        reader.read(buf)
    } else {
        usize::MAX
    };
    drop(process);
    inner.context.x[10] = value;
    Ok(())
}

// 写入
pub fn sys_write(task: SyscallTask, fd: usize, buf_ptr: UserAddr<u8>, count: usize) -> Result<(), RuntimeError> {
    let buf = buf_ptr.transfer_vec(count);
    let mut inner = task.inner.borrow_mut();
    let mut process = inner.process.borrow_mut();
    
    // 判断文件描述符是否存在
    let writer = process.fd_table.get(fd)?;
    let value = if writer.writeable() {
        writer.write(buf, buf.len())
    } else {
        usize::MAX
    };
    drop(process);
    inner.context.x[10] = value;
    Ok(())
}
// 写入
pub fn sys_writev(task: SyscallTask, fd: usize, iov: UserAddr<IoVec>, iovcnt: usize) -> Result<(), RuntimeError> {
    let iov_vec = iov.transfer_vec(iovcnt);
    
    let mut inner = task.inner.borrow_mut();
    let mut process = inner.process.borrow_mut();
    
    let fd = process.fd_table.get(fd)?;
    let mut cnt = 0;
    for i in iov_vec {
        // let buf = get_buf_from_phys_addr(i.iov_base.translate(process.pmm.clone()), 
        //     i.iov_len);
        let buf = i.iov_base.transfer_vec(i.iov_len);
        cnt += fd.write(buf, i.iov_len);
    }
    drop(process);
    inner.context.x[10] = cnt;
    Ok(())
}

pub fn sys_readv(task: SyscallTask, fd: usize, iov: UserAddr<IoVec>, iovcnt: usize) -> Result<(), RuntimeError> {
    let iov_vec = iov.transfer_vec(iovcnt);

    let mut inner = task.inner.borrow_mut();
    let mut process = inner.process.borrow_mut();
    
    let fd = process.fd_table.get(fd)?;
    let mut cnt = 0;
    for i in iov_vec {
        // let buf = get_buf_from_phys_addr(i.iov_base, 
            // i.iov_len);
        let buf = i.iov_base.transfer_vec(i.iov_len);
        cnt += fd.read(buf);
    }
    drop(process);
    inner.context.x[10] = cnt;
    Ok(())
}

pub fn sys_lseek(task: SyscallTask, fd: usize, offset: usize, whence: usize) -> Result<(), RuntimeError> {
    let mut inner = task.inner.borrow_mut();
    let mut process = inner.process.borrow_mut();

    let file = process.fd_table.get(fd)?;
    let offset = file.lseek(offset, whence);
    // debug!("lseek Filename: {}", file.get_inode().get_filename());
    // let inode = file.get_inode();
    drop(process);
    inner.context.x[10] = offset;
    Ok(())
}

// 原子读
pub fn sys_pread(task: SyscallTask, fd: usize, ptr: UserAddr<u8>, len: usize, offset: usize) -> Result<(), RuntimeError> {
    let buf = ptr.transfer_vec(len);
    let mut inner = task.inner.borrow_mut();
    let process = inner.process.borrow_mut();
    let file = process.fd_table.get_file(fd)?;
    let ret = file.read_at(offset, buf);
    drop(process);
    inner.context.x[10] = ret;
    Ok(())
}

pub fn sys_sendfile(task: SyscallTask, out_fd: usize, in_fd: usize, offset_ptr: usize, count: usize) -> Result<(), RuntimeError> {
    let mut inner = task.inner.borrow_mut();
    let mut process = inner.process.borrow_mut();
    let in_file = process.fd_table.get(in_fd)?;
    let size = in_file.get_size();
    let mut buf = vec![0u8; size];
    let read_size = in_file.read(&mut buf);

    let out_file = process.fd_table.get(out_fd)?;
    out_file.write(&buf, buf.len());

    drop(process);
    inner.context.x[10] = read_size;
    Ok(())
}