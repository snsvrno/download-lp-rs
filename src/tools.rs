pub fn split_name_and_extension<'a>(file : &'a str) -> (&'a str,&'a str) {
    //! splits a url into the filename and the path, all references

    let extension : &'a str = if let Some (ext) = file.split(".").last() { 
        ext 
    } else { 
            "" 
    };

    let filename : &'a str = if let Some (filename) = file.split("/").last() { 
        if let Some(filename) = filename.split("\\").last() {
            &filename[..(filename.len()-extension.len()-1)]
        } else {
            &filename[..(filename.len()-extension.len()-1)]
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

        let (file2,ext2) = super::split_name_and_extension("how\\about\\this.one");
        assert_eq!(file2,"this");
        assert_eq!(ext2,"one");
    }

}