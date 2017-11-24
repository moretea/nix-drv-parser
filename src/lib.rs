use std::path::Path; use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod drv_grammar {
    include!(concat!(env!("OUT_DIR"), "/drv_grammar.rs"));
}

#[derive(Debug)]
pub struct Derivation {
    pub outputs: Vec<Output>,
    pub inputs: Vec<DerivationRef>,
    pub input_sources: Vec<String>,
    pub platform: String,
    pub builder: String,
    pub builder_arguments: Vec<String>,
    pub environment: HashMap<String,String>,
}

#[derive(Debug)]
pub struct Output {
    pub id: String,
    pub path: String,
    pub hash_algorithm: String,
    pub hash: String,
}

#[derive(Debug)]
pub struct DerivationRef {
    path: String,
    outputs: Vec<String>
}

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(drv_grammar::ParseError)
}

impl std::convert::From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Error {
        Error::IOError(io_error)
    }
}
impl std::convert::From<drv_grammar::ParseError> for Error {
    fn from(parse_error: drv_grammar::ParseError) -> Error {
        Error::ParseError(parse_error)
    }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Derivation, Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;

    parse(contents)
}

pub fn parse(derivation: String) -> Result<Derivation,Error> {
    Ok(drv_grammar::derivation(&derivation)?)
}
