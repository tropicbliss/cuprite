#![warn(clippy::pedantic)]

mod cli;
mod file_io;
mod minecraft;

use anyhow::{bail, Context, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::new();
    let manipulator = file_io::FileManipulator::new(
        args.input_dir,
        args.output_dir,
        usize::from(args.max_backups),
        args.compression_level,
    );
    if let Err(e) = manipulator
        .truncate_target_dir()
        .with_context(|| "Failed to truncate target directory")
    {
        let mut server = minecraft::Server::new(args.rcon_port, args.rcon_password)
            .await
            .with_context(|| "Failed to connect to server")?;
        server
            .connect()
            .await
            .with_context(|| "Failed to send initial RCON messages")?;
        server
            .disconnect(false)
            .await
            .with_context(|| "Failed to send final RCON messages")?;
        bail!(e);
    }
    let mut server = minecraft::Server::new(args.rcon_port, args.rcon_password)
        .await
        .with_context(|| "Failed to connect to server")?;
    server
        .connect()
        .await
        .with_context(|| "Failed to send initial RCON messages")?;
    match manipulator.read_to_zip() {
        Ok(_) => {
            server
                .disconnect(true)
                .await
                .with_context(|| "Failed to send final RCON messages")?;
        }
        Err(e) => {
            server
                .disconnect(false)
                .await
                .with_context(|| "Failed to send final RCON messages")?;
            bail!(e);
        }
    }
    Ok(())
}
