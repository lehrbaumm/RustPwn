use pwnd::tubes::{
    create_process,
    Tube
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut p: Box<dyn Tube> = create_process("python", Some(["-c", "print('hello world');user_input=input();print(user_input)"].to_vec()), None)?;
    println!("{:?}", String::from_utf8_lossy(&p.recvuntil(b"\r\n", false, None).await?));
    println!("{:?}", p.sendline(b"Test", None).await);
    println!("{:?}", String::from_utf8_lossy(&p.recvline(false, None).await?));
    p.print_status();
    Ok(())
}
