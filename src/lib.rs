extern crate lpsettings;
extern crate reqwest;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub enum Error {
  Reqwest(reqwest::Error),
  Io(io::Error)
}

pub fn download(link : String) -> Result<(PathBuf,usize),Error> {
  let (file,ext) = split_name_and_extension(&link);
  
  // builds the download path
  let mut download_path = if let Ok(settings_path) = lpsettings::get_settings_folder() { settings_path } else { PathBuf::from(".") };
  download_path.push(lpsettings::get_value_or("cache.path","cache"));
  download_path.push(format!("{}.{}",file,ext));

  // downloads the data
  let mut buffer : Vec<u8> = Vec::new();
  match reqwest::get(&link) {
    Err(error) => { return Err(Error::Reqwest(error)); }
    Ok(mut link_data) => { 
      match link_data.copy_to(&mut buffer) {
        Err(error) => { return Err(Error::Reqwest(error)); }
        Ok(_) => { }
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
    if let Some(filename) = filename.split(".").next() {
      filename
    } else { 
      filename 
    }
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