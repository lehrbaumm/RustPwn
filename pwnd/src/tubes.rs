use async_trait::async_trait;

pub mod process;

#[async_trait]
pub trait Tube {
    fn print_status(&self);
    async fn recvline(&mut self, keep_end: bool, timeout: Option<u64>) -> tokio::io::Result<Vec<u8>>;
    async fn recvuntil<'a>(&mut self, pattern: &'a [u8], keep_end: bool, timeout: Option<u64>) -> tokio::io::Result<Vec<u8>>;
    async fn sendline<'a>(&mut self, input: &'a [u8], timeout: Option<u64>) -> tokio::io::Result<usize>; 
    async fn send<'a >(&mut self, input: &'a [u8], timeout: Option<u64>) -> tokio::io::Result<usize>;
    async fn interactive(&mut self) -> tokio::io::Result<()>;
    async fn close(&mut self) -> tokio::io::Result<()>;
}

//pub async fn process()
