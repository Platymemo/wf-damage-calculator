use std::{io::{self, Cursor, Read}, path::Path};
use lzma;

const MANIFEST_URL: &str = "https://origin.warframe.com/PublicExport/index_en.txt.lzma";
const MANIFEST_PATH: &str = "./index_en.txt.lzma";

// Returns the newline separated list of all exportable categories.
pub fn get_manifest() -> Result<String, BinError> {
    let lzma = Path::new(MANIFEST_PATH);
    if !lzma.exists() {
        download_binary(MANIFEST_URL, lzma)?;
    }

    let mut output = String::new();
    let _ = lzma::open(lzma)?.read_to_string(&mut output);

    Ok(output)
}

#[derive(Debug)]
pub enum BinError {
    Reqwest(reqwest::Error),
    Io(io::Error),
    Lzma(lzma::Error)
}

impl From<reqwest::Error> for BinError {
    fn from(err: reqwest::Error) -> Self {
        BinError::Reqwest(err)
    }
}

impl From<io::Error> for BinError {
    fn from(err: io::Error) -> Self {
        BinError::Io(err)
    }
}

impl From<lzma::Error> for BinError {
    fn from(err: lzma::Error) -> Self {
        BinError::Lzma(err)
    }
}

// DANGEROUS!!!
// Make sure the URL is trusted!
fn download_binary<P: AsRef<Path>>(url: &str, path: P) -> Result<(), BinError> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(path)?;
    let mut content =  Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}
