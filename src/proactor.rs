use io_uring::{IoUring, opcode, types};
use std::io;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::future::poll_fn;
use std::os::fd::RawFd;

///
pub struct Proactor {
    uring: Arc<Mutex<IoUring>>,
}

impl Proactor {
    ///
    pub fn new(entries: u32) -> io::Result<Self> {
        let uring = IoUring::new(entries)?;
        Ok(Self { uring: Arc::new(Mutex::new(uring)) })
    }

    ///
    pub async fn read(&self, fd: RawFd, buf: &mut [u8]) -> io::Result<usize> {
        {
            let uring = self.uring.clone();
            let entry = opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), buf.len() as _).build();
            let mut uring_guard = uring.lock().unwrap();
            unsafe {
                uring_guard.submission().push(&entry).expect("Failed to submit request");
            }
            uring_guard.submit().expect("Failed to submit io_uring");
        }

        poll_fn(|cx| {
            let mut uring_guard = self.uring.lock().unwrap();
            if let Some(cqe) = uring_guard.completion().next() {
                let result = cqe.result();
                if result < 0 {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(-result)));
                } else {
                    return Poll::Ready(Ok(result as usize));
                }
            }
            let waker = cx.waker().clone();
            uring_guard.submit_and_wait(1).expect("Failed to wait");
            waker.wake();

            Poll::Pending
        }).await
    }
}