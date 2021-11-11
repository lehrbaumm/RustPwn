use async_trait::async_trait;
use std::collections::HashMap;

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

pub fn create_process(file_name: &str, args: Option<Vec<&str>>, env: Option<HashMap<String, String>>) -> tokio::io::Result<Box<Tube>> {
    use process::ProcessBuilder;
    let mut process_builder = &mut ProcessBuilder::new(file_name)?;
    if let Some(arguments) = args {
        for arg in arguments {
            process_builder = process_builder.arg(arg);
        }
    }
    if let Some(environment) = env {
        process_builder = process_builder.set_env(environment);
    }
    Ok(Box::new(process_builder.build()?))
}
