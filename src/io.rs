use crate::Proactor;
use std::io::{self};
use std::os::fd::AsRawFd;

pub struct Stdin<'r> {
    proactor: &'r Proactor,
    stdin: io::Stdin,
}

impl<'r> Stdin<'r> {
    pub fn new(proactor: &'r Proactor) -> io::Result<Self> {
        Ok(Self {
            proactor,
            stdin: io::stdin(),
        })
    }

    pub async fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.proactor.read(self.stdin.as_raw_fd(), buf).await
    }
}
