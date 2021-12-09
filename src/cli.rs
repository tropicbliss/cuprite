use std::num::NonZeroUsize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author, about)]
pub struct Args {
    /// Input directory (directory to backup)
    #[structopt(short = "i", long, default_value = "world")]
    pub input_dir: PathBuf,

    /// Output directory
    #[structopt(short = "o", long, default_value = "backups")]
    pub output_dir: PathBuf,

    /// Maximum number of backups to keep
    #[structopt(short = "m", long, default_value = "128")]
    pub max_backups: NonZeroUsize,

    /// RCON port
    #[structopt(short = "P", long = "port", default_value = "25575")]
    pub rcon_port: u16,

    /// RCON password
    #[structopt(short = "p", long = "password", default_value = "")]
    pub rcon_password: String,

    /// Compression level
    #[structopt(short = "c", , default_value = "3")]
    pub compression_level: u32,
}

impl Args {
    pub fn new() -> Self {
        Self::from_args()
    }
}
