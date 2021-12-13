use anyhow::{bail, Context, Result};
use chrono::{Local, TimeZone, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{create_dir_all, read_dir, remove_file, File};
use std::io::BufWriter;
use std::num::NonZeroUsize;
use std::path::PathBuf;

const DATE_FORMAT: &str = "%Y-%m-%d-%H-%M-%S";

pub struct FileManipulator {
    input_dir: PathBuf,
    output_dir: PathBuf,
    max_backups: NonZeroUsize,
    compression_level: u32,
}

impl FileManipulator {
    pub fn new(
        input_dir: PathBuf,
        output_dir: PathBuf,
        max_backups: NonZeroUsize,
        compression_level: u32,
    ) -> Self {
        Self {
            input_dir,
            output_dir,
            max_backups,
            compression_level,
        }
    }

    pub fn read_to_zip(&self) -> Result<()> {
        if !self.input_dir.is_dir() {
            bail!("Input path is not a directory");
        }
        if !self.output_dir.is_dir() {
            bail!("Output path is not a directory");
        }
        self.zip_dir()
            .with_context(|| "Failed to compress directory into tarball")?;
        Ok(())
    }

    fn zip_dir(&self) -> Result<()> {
        let mut output_path = self.output_dir.clone();
        output_path.push(format!(
            "Backup-{}.tar.gz",
            Local::now().format(DATE_FORMAT)
        ));
        let tar_gz = BufWriter::new(File::create(output_path)?);
        let enc = GzEncoder::new(tar_gz, Compression::new(self.compression_level));
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(&self.input_dir, &self.input_dir)?;
        tar.finish()?;
        Ok(())
    }

    pub fn truncate_target_dir(&self) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^Backup-.*tar.gz$").unwrap();
        }
        create_dir_all(&self.output_dir)?;
        let mut result = Vec::new();
        for path in read_dir(&self.output_dir)? {
            let path = path?;
            if path.file_type()?.is_file() {
                if let Some(file_name) = path.file_name().to_str() {
                    if RE.is_match(file_name)
                        && Utc
                            .datetime_from_str(&file_name[7..26], DATE_FORMAT)
                            .is_ok()
                    {
                        path.metadata()?;
                        path.metadata().unwrap().modified()?;
                        result.push(path);
                    }
                }
            }
        }
        result.sort_by_key(|path| path.metadata().unwrap().modified().unwrap());
        let max_backups = usize::from(self.max_backups);
        if result.len() >= max_backups {
            let surplus = result.len() - max_backups + 1;
            let files_to_remove = &result[0..surplus];
            for file in files_to_remove {
                remove_file(file.path())?;
            }
        }
        Ok(())
    }
}
