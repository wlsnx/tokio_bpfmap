use std::ops::{Deref, DerefMut};

use tokio::io::unix::AsyncFd;

pub trait EPollFd {
    fn epoll_fd(&self) -> i32;
}

impl EPollFd for libbpf_rs::PerfBuffer<'_> {
    fn epoll_fd(&self) -> i32 {
        self.epoll_fd()
    }
}

impl EPollFd for libbpf_rs::RingBuffer<'_> {
    fn epoll_fd(&self) -> i32 {
        self.epoll_fd()
    }
}

pub struct AsyncBuffer<T: EPollFd> {
    inner: T,
    async_fd: AsyncFd<i32>,
}

impl<T: EPollFd> Deref for AsyncBuffer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: EPollFd> DerefMut for AsyncBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: EPollFd> AsyncBuffer<T> {
    pub fn new(inner: T) -> std::io::Result<Self> {
        let fd = inner.epoll_fd();
        Ok(Self {
            inner,
            async_fd: AsyncFd::new(fd)?,
        })
    }

    pub async fn readable(&self) -> std::io::Result<()> {
        loop {
            let g = self.async_fd.readable().await?;
            let ready = g.ready();
            if ready.is_readable() {
                return Ok(());
            }
        }
    }
}
