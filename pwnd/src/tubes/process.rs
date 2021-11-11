use tokio::process::{Command, Child, ChildStdout, ChildStdin, ChildStderr};
use std::process::{Stdio, ExitStatus};
use std::convert::Into;
use std::collections::HashMap;
use std::time::Duration;
use super::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time;

use crate::tubes::Tube;

#[derive(Debug)]
pub struct Process {
    file_name: String,
    process: Option<Child>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
    stdin: Option<ChildStdin>,
    exit_code: Option<ExitStatus>
}

pub struct ProcessBuilder {
    file_name: String,
    args: Vec<String>,
    env: Option<HashMap<String, String>>
}

impl ProcessBuilder {
    pub fn new<T: Into<String>>(file_name: T) -> tokio::io::Result<Self> {
        Ok(Self {
            file_name: file_name.into(),
            args: Vec::new(),
            env: None
        })
    }

    pub fn arg<'a, T: Into<String>>(&'a mut self, arg: T) -> &'a mut ProcessBuilder {
        self.args.push(arg.into());
        self
    }

    pub fn set_env<'a>(&'a mut self, env: HashMap<String, String>) -> &'a mut ProcessBuilder {
        self.env = Some(env);
        self
    }

    pub fn build(&mut self) -> tokio::io::Result<Process> {
        let mut bare_command = Command::new(&self.file_name);
        let mut command: &mut Command = bare_command.stdin(Stdio::piped()).stdout(Stdio::piped());//.stderr(Stdio::piped());
        match &self.env {
            None => (),
            Some(env) => {
                command = command.env_clear().envs(env);
            }
        }
        for arg in &self.args {
            command = command.arg(arg);
        }
        let mut child = command.spawn()?;
        let stdout = child.stdout.take();
        let stdin = child.stdin.take();
        let stderr = child.stderr.take();
        Ok(Process {
            file_name: String::from(&self.file_name),
            process: Some(child),
            stdout,
            stderr,
            stdin,
            exit_code: None
        })
    }
}

impl Process {
    async fn check_process(&mut self) {
        match self.process.as_mut() {
            None => (),
            Some(child_process) => {
                let wait_function = child_process.wait();
                match time::timeout(Duration::from_millis(30), wait_function).await {
                    Ok(status) => match status {
                        Ok(exit_code) => {
                            self.process = None;
                            self.exit_code = Some(exit_code);
                        },
                        Err(_) => ()
                    },
                    Err(_) => ()
                }
            }
        }
    }
}

#[async_trait]
impl Tube for Process {
    fn print_status(&self) {
        match self.process {
            Some(_) => println!("Process running."),
            None => println!("Process exited with exit_code {}", self.exit_code.unwrap())
        };
    }

    async fn close(&mut self) -> tokio::io::Result<()> {
        self.check_process().await;
        match self.process.as_mut() {
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No running process.")),
            Some(child_process) => child_process.kill().await
        }
    }

    async fn recvline(&mut self, keep_end: bool, timeout: Option<u64>) -> tokio::io::Result<Vec<u8>> {
        self.recvuntil(b"\n", keep_end, timeout).await
    }

    async fn recvuntil<'a>(&mut self, pattern: &'a [u8], keep_end: bool, timeout: Option<u64>) -> tokio::io::Result<Vec<u8>> {
        self.check_process().await;
        let real_timeout = match timeout {
            None => 300,
            Some(n) => n
        };
        let mut result: Vec<u8> = Vec::with_capacity(1024);
        let stdout = self.stdout.as_mut().unwrap();
        loop {
            let mut buffer = [0; 1];

            let read_function = stdout.read(&mut buffer[..]);
            let bytes_read = time::timeout(Duration::from_millis(real_timeout), read_function).await??;
            //let _ = stdout.read(&mut buffer[..]).await?;

            if bytes_read == 0 && self.process.is_none() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Process exited!"));
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

    async fn sendline<'a>(&mut self, input: &'a [u8], timeout: Option<u64>) -> tokio::io::Result<usize> {
        let mut new_input: Vec<u8> = Vec::from(input);
        new_input.push(b'\n');
        self.send(&new_input, timeout).await
    }

    async fn send<'a>(&mut self, input: &'a [u8], timeout: Option<u64>) -> tokio::io::Result<usize> {
        self.check_process().await;
        let real_timeout = match timeout {
            None => 300,
            Some(n) => n
        };
        let stdin = self.stdin.as_mut().unwrap();
        let write_function = stdin.write(input.into());

        let bytes_written = time::timeout(Duration::from_millis(real_timeout), write_function).await??;

        if bytes_written != input.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not enough written!"));
        }

        Ok(bytes_written)
    }

    async fn interactive(&mut self) -> tokio::io::Result<()> {
        todo!()
    }
}
