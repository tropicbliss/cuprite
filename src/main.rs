#![warn(clippy::pedantic)]

mod cli;
mod file_io;
mod minecraft;

use anyhow::{bail, Context, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::new();
    let mut server = minecraft::Server::new(args.rcon_port, args.rcon_password)
        .await
        .with_context(|| "Failed to connect to server")?;
    server
        .connect()
        .await
        .with_context(|| "Failed to send initial RCON messages")?;
    let manipulator = file_io::FileManipulator::new(
        args.input_dirs,
        args.output_dir,
        args.max_backups,
        args.compression_level,
    );
    if let Err(e) = manipulator
        .truncate_target_dir()
        .with_context(|| "Failed to truncate target directory")
    {
        server
            .disconnect(false)
            .await
            .with_context(|| "Failed to send final RCON messages")?;
        bail!(e);
    }
    match manipulator
        .read_to_zip()
        .with_context(|| "Failed to compress directory into tarball")
    {
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
