#![warn(clippy::pedantic)]

mod cli;
mod file_io;
mod minecraft;
mod rcon;

use anyhow::{bail, Context, Result};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let args = cli::Args::new();
    let manipulator = file_io::FileManipulator::new(
        args.input_dir,
        args.output_dir,
        usize::from(args.max_backups),
    );
    match manipulator
        .truncate_target_dir()
        .with_context(|| "Failed to truncate target directory")
    {
        Ok(_) => writeln!(stdout(), "Successfully truncated directory")?,
        Err(e) => {
            let mut server = minecraft::Server::new(args.rcon_port, args.rcon_password)
                .with_context(|| "Failed to connect to server")?;
            server
                .connect()
                .with_context(|| "Failed to send initial RCON messages")?;
            server
                .disconnect(false)
                .with_context(|| "Failed to send final RCON messages")?;
            bail!(e);
        }
    }
    let mut server = minecraft::Server::new(args.rcon_port, args.rcon_password)
        .with_context(|| "Failed to connect to server")?;
    server
        .connect()
        .with_context(|| "Failed to send initial RCON messages")?;
    match manipulator.rw_zip() {
        Ok(_) => {
            writeln!(stdout(), "Successfully backed up directory")?;
            server
                .disconnect(true)
                .with_context(|| "Failed to send final RCON messages")?;
        }
        Err(e) => {
            server
                .disconnect(false)
                .with_context(|| "Failed to send final RCON messages")?;
            bail!(e);
        }
    }
    Ok(())
}
