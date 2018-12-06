extern crate download_lp as download;

extern crate log;
extern crate pretty_env_logger;

static LINK : &'static str = "https://github.com/rust-lang/book/archive/one-book-to-rule-them-all.zip";

fn main() {

    // starts the loggers & sets the filter level for the logs
    let mut builder = pretty_env_logger::formatted_builder();
    builder
        .filter(None,log::LevelFilter::Info )
        .init();

    let result = download::download(LINK,".");
}
