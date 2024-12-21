use crate::Proactor;
use std::fs::File;
use std::io::{self, SeekFrom};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;

pub struct AsyncFile<'r> {
    proactor: &'r Proactor,
    file: File,
}

impl<'r> AsyncFile<'r> {
    // 打开文件
    pub fn open<P: AsRef<Path>>(proactor: &'r Proactor, path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self { proactor, file })
    }

    // 异步读取文件
    pub async fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.proactor.read(self.file.as_raw_fd(), buf).await
    }

    // 异步写入文件
    pub async fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.proactor.write(self.file.as_raw_fd(), buf).await
    }

    // 获取底层文件描述符
    pub fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}
