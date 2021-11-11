use pwnd::tubes::{process::ProcessBuilder, Tube};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut p: Box<dyn Tube> = Box::new(
        ProcessBuilder::new("python")?.arg("-c").arg("print('hello world');user_input=input();print(user_input)").build()?
    );
    //println!("{:?}", String::from_utf8_lossy(&p.recvline(false, None).await?));
    println!("{:?}", String::from_utf8_lossy(&p.recvuntil(b"\r\n", false, None).await?));
    println!("{:?}", p.sendline(b"Test", None).await);
    println!("{:?}", String::from_utf8_lossy(&p.recvline(false, None).await?));
    p.print_status();
    Ok(())
}
