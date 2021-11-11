use std::convert::Into;
use std::time::Duration;
use std::error::Error;
use super::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;

use super::Tube;

#[derive(Debug)]
pub struct TCP {
    hostname: String,
    port: u16,
    stream: Option<TcpStream>
}

impl TCP {
    pub fn new<T: Into<String>, U: Into<u16>>(hostname: T, port: U) -> Self {
        Self {
            hostname: hostname.into(),
            port: port.into(),
            stream: None
        }
    }

    pub async fn connect<'a>(&'a mut self) -> Result<(), Box<dyn Error>> {
        self.stream = Some(TcpStream::connect(format!("{}:{}", self.hostname, self.port)).await?);
        Ok(())
    }
}

#[async_trait]
impl Tube for TCP {
    async fn close(&mut self) -> tokio::io::Result<()> {
        match self.stream.as_mut() {
            None => Ok(()),
            Some(connection) => connection.shutdown().await
        }
    }

    fn print_status(&self) {
        todo!()
    }

    async fn recvline(&mut self, keep_end: bool, timeout: Option<u64>) -> tokio::io::Result<Vec<u8>> {
        self.recvuntil(b"\n", keep_end, timeout).await
    }

    async fn recvuntil<'a>(&mut self, pattern: &'a [u8], keep_end: bool, timeout: Option<u64>) -> tokio::io::Result<Vec<u8>> {
        let real_timeout = match timeout {
            None => 10,
            Some(n) => n
        };
        match self.stream.as_mut() {
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No connection.")),
            Some(connection) => {
                let mut result: Vec<u8> = Vec::with_capacity(1024);
                loop {
                    let mut buffer = [0; 1];

                    let read_function = connection.read(&mut buffer[..]);
                    let bytes_read = time::timeout(Duration::from_secs(real_timeout), read_function).await??;
                    //let _ = stdout.read(&mut buffer[..]).await?;

                    if bytes_read == 0 {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Connection lost!"));
                    }
                    result.push(buffer[0]);

                    if let Some(index) = result.iter().position(|&i| i == pattern[0]) {
                        if index + pattern.len() == result.len() {
                            let mut pattern_found = true;
                            for i in 1..pattern.len() {
                                if pattern[i] != result[index+i] {
                                pattern_found = false;
                                }
                            }
                            if pattern_found {
                                break;
                            }
                        }
                    }
                }
                if !keep_end {
                    for _ in 0..pattern.len() {
                        result.pop();
                    }
                }
                Ok(result)
            }
        }
    }

    async fn sendline<'a>(&mut self, input: &'a [u8], timeout: Option<u64>) -> tokio::io::Result<usize> {
        let mut new_input: Vec<u8> = Vec::from(input);
        new_input.push(b'\n');
        self.send(&new_input, timeout).await
    }

    async fn send<'a >(&mut self, input: &'a [u8], timeout: Option<u64>) -> tokio::io::Result<usize> {
        let real_timeout = match timeout {
            None => 10,
            Some(n) => n
        };
        match self.stream.as_mut() {
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No connection.")),
            Some(connection) => {
                let write_function = connection.write(input);
                let bytes_written = time::timeout(Duration::from_secs(real_timeout), write_function).await??;

                if bytes_written != input.len() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not enough written!"));
                }

                Ok(bytes_written)
            }
        }
    }

    async fn interactive(&mut self) -> tokio::io::Result<()> {
        todo!()
    }
}