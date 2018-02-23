extern crate lpsettings;
extern crate reqwest;
extern crate ansi_term; use ansi_term::Colour::{Yellow,Blue};
#[macro_use]
extern crate output;

use std::fs::{File,create_dir_all};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub enum Error {
  Reqwest(reqwest::Error),
  Io(io::Error)
}

pub fn download(link : String) -> Result<(PathBuf,usize),Error> {
  //! downloads the file to the lpsettings defined cache and returns that path

  output_println!("Downloading {}",&link);

  let (file,ext) = split_name_and_extension(&link);
  
  // builds the download path
  let mut download_path = if let Ok(settings_path) = lpsettings::get_settings_folder() { settings_path } else { PathBuf::from(".") };
  download_path.push(lpsettings::get_value_or("cache.path","cache"));
  if !download_path.exists() { 
    output_debug!("{} does not exists!",download_path.display().to_string());
    match create_dir_all(&download_path) {
      Ok(_) => { output_debug!("Created folder"); }
      Err(error) => { output_debug!("Could not create folder: {}",Yellow.paint(error.to_string()))}
    }
  }
  download_path.push(format!("{}.{}",file,ext));
  output_debug!("Download path is {}",&download_path.display().to_string());

  if download_path.exists() {
    output_println!("{} already exist, skipping download.",Blue.paint(download_path.display().to_string()));
    return Ok((download_path,0));
  }

  // downloads the data
  let mut buffer : Vec<u8> = Vec::new();
  match reqwest::get(&link) {
    Err(error) => { 
      output_error!("Couldn't get request: {}",Yellow.paint(error.to_string()));
      return Err(Error::Reqwest(error)); }
    Ok(mut link_data) => { 
      match link_data.copy_to(&mut buffer) {
        Err(error) => { 
          output_error!("Couldn't copy response to buffer: { }",Yellow.paint(error.to_string()));
          return Err(Error::Reqwest(error)); }
        Ok(_) => { output_println!("{} finished downloading.",&link); }
      }
    }
  }

  match File::create(&download_path) {
    Err(error) => { return Err(Error::Io(error)); }
    Ok(mut download_file) => { 
      match download_file.write(&buffer) {
        Err(error) => { return Err(Error::Io(error)); }
        Ok(size) => { return Ok((download_path,size)); }
      }
    }
  }

}

fn split_name_and_extension<'a>(file : &'a str) -> (&'a str,&'a str) {
  //! splits a url into the filename and the path

  let extension : &'a str = if let Some (ext) = file.split(".").last() { 
    ext 
  } else { "" };

  let filename : &'a str = if let Some (filename) = file.split("/").last() { 
    &filename[..(filename.len()-extension.len()-1)]
  } else { file };

  (filename,extension)
}

///////////////////////////////////////

mod test {
  #[test]
  fn split_name_and_extension() {
    let (file,ext) = super::split_name_and_extension("what/is/this/file.name");
    assert_eq!(file,"file");
    assert_eq!(ext,"name");
  }

}