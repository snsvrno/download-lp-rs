extern crate download_lp as download;

static LINK : &'static str = "https://github.com/rust-lang/book/archive/one-book-to-rule-them-all.zip";

fn main() {
    let result = download::download(LINK,".");
    
    println!("{:?}",result);
}
