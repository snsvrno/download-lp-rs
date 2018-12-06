extern crate reqwest;
extern crate failure; use failure::Error;
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

    // does the actual downloading.
    let mut buffer : Vec<u8> = Vec::new();

    let mut response = reqwest::get(link)?;
    response.copy_to(&mut buffer)?;

    // now writes it to a file
    let mut disk_file = fs::File::create(&download_path)?;
    let size = disk_file.write(&buffer)?;

    info!("Download complete!");
    
    Ok(size)
}