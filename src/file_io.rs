use anyhow::{bail, Context, Result};
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{create_dir_all, read_dir, remove_file, File};
use std::path::PathBuf;

pub struct FileManipulator {
    input: PathBuf,
    output: PathBuf,
    max_files: usize,
    compression_level: u32,
}

impl FileManipulator {
    pub fn new(input: PathBuf, output: PathBuf, max_files: usize, compression_level: u32) -> Self {
        Self {
            input,
            output,
            max_files,
            compression_level,
        }
    }

    pub fn read_to_zip(&self) -> Result<()> {
        if !self.input.is_dir() {
            bail!("Input path is not a directory");
        }
        if !self.output.is_dir() {
            bail!("Output path is not a directory");
        }
        self.zip_dir()
            .with_context(|| "Failed to compress directory into tarball")?;
        Ok(())
    }

    fn zip_dir(&self) -> Result<()> {
        let mut output_path = self.output.clone();
        output_path.push(format!(
            "Backup-{}.tar.gz",
            Local::now().format("%Y-%m-%d-%H-%M-%S")
        ));
        let tar_gz = File::create(output_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::new(self.compression_level));
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(&self.input, &self.input)?;
        tar.finish()?;
        Ok(())
    }

    pub fn truncate_target_dir(&self) -> Result<()> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Backup-\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2}.tar.gz$").unwrap();
        }
        create_dir_all(&self.output)?;
        let mut result = Vec::new();
        for path in read_dir(&self.output)? {
            let path = path?;
            if path.file_type()?.is_file() {
                if let Some(file_name) = path.file_name().to_str() {
                    if RE.is_match(file_name)
                        && path.metadata().is_ok()
                        && path.metadata().unwrap().modified().is_ok()
                    {
                        result.push(path);
                    }
                }
            }
        }
        result.sort_by_key(|path| path.metadata().unwrap().created().unwrap());
        if result.len() >= self.max_files {
            let surplus = result.len() - self.max_files + 1;
            let files_to_remove = &result[0..surplus];
            for file in files_to_remove {
                remove_file(file.path())?;
            }
        }
        Ok(())
    }
}
