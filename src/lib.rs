extern crate reqwest;
extern crate indicatif;
#[macro_use] extern crate failure; use failure::Error;
#[macro_use] extern crate log;

use std::path::{ Path, PathBuf };
use std::fs;
use std::io::prelude::*;

mod tools;

pub fn download<P:AsRef<Path>>(link : &str, path : P) -> Result<usize,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display,
{
    //! No frills downloader.
    //! 
    //! Pick a link and a path and it will download to that path. Designed to be 
    //! a super simple way to quickly download a file.AsMut
    //! 
    //! Uses [log](https://crates.io/crates/log) so some output is available for 
    //! debugging purposes. 

    info!("Downloading '{}' to path '{}'",link,path);

    let (file,ext) = tools::split_name_and_extension(link);

    let mut download_path : PathBuf = PathBuf::from(path);
    // checks if the download path exists, and tries to create the folders if it doesn't
    if !download_path.exists() {
        warn!("Download path of '{:?}' doesn't exist, attempting to create it.",&download_path);
        fs::create_dir_all(&download_path)?;
        info!("Folders created successfully");
    }

    download_path.push(format!("{}.{}",file,ext));

    if download_path.exists() {
        info!("File already seems to exist, skipping download.");
        return Ok(0);
    }

     let client = reqwest::Client::new();
    let mut resp = client.get(link).send()?;

    if resp.status().is_success() {
        let len : u64 = resp.headers().get(reqwest::header::CONTENT_LENGTH)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok())
            .unwrap_or(0);
        
        // let progress = indicatif::ProgressBar::new(len);
        let progress = indicatif::ProgressBar::new_spinner();
        // progress.println(format!("Downloading: {}",file));
        progress.set_message(&format!("Downloading: {}",file));

        let chunk_size = 1024usize;

        let mut buffer : Vec<u8> = Vec::new();
        let mut total : usize = 0;

        loop {
            let mut small_buffer = vec![0; chunk_size];
            let small_buffer_read = resp.read(&mut small_buffer[..])?;
            small_buffer.truncate(small_buffer_read);

            match small_buffer.is_empty() {
                true => break,
                false => {
                    buffer.extend(small_buffer);
                    total += small_buffer_read;
                    //progress.inc(small_buffer_read as u64);
                    progress.set_message(&format!("{}.{} : {} / {}",file,ext,total,len));
                },
            }
        }

        let mut disk_file = fs::File::create(&download_path)?;
        let size_disk = disk_file.write(&buffer)?;

        progress.finish_with_message(&format!("{}.{} : Done",file,ext));

        Ok(size_disk)

    } else {
        Err(format_err!("No response, are you connected to the internet?"))
    }
}