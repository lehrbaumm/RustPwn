use pwnd::tubes::{
    create_process,
    Tube
};
use pwnd::util::packing::{pack, unpack, ENDIANNESS};
use tracing::info;
use color_eyre::Report;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    setup()?;

    let mut p: Box<dyn Tube> = create_process("python", Some(["-c", "print('hello world');user_input=input();print(user_input)"].to_vec()), None)?;
    info!("{:?}", String::from_utf8_lossy(&p.recvuntil(b"\r\n", false, None).await?));
    info!("{:?}", p.sendline(b"Test", None).await);
    info!("{:?}", String::from_utf8_lossy(&p.recvline(false, None).await?));
    info!("{:X?}", pack(1234585, 64, ENDIANNESS::LITTLE));
    info!("{:?}", unpack([0x99, 0xd6, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec(), 64, ENDIANNESS::LITTLE));
    p.print_status();
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
