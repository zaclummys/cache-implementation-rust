pub mod error;
pub mod header;
pub mod parser;

use std::fs::File;
use std::path::Path;

use std::io::{
    Read,
    Lines,
    BufRead,
    BufReader,
};

pub use self::error::SourceError;
pub use self::header::SourceHeader;

use self::parser::{
    decimal,
    hexadecimal,
};

type SourceResult<T> = Result<T, SourceError>;

pub struct Source<T> {
    lines: Lines<T>
}

impl Source<BufReader<File>> {
    pub fn open<P: AsRef<Path>> (path: P) -> Result<Source<BufReader<File>>, std::io::Error> {
        Ok(Source::new(BufReader::new(File::open(path)?)))
    }
}

impl<R: BufRead + Read> Source<R> {
    pub fn new (reader: R) -> Source<R> {
        Source {
            lines: reader.lines(),
        }
    }

    fn line (&mut self) -> SourceResult<Option<String>> {
        match self.lines.next() {
            None => Ok(None),
            Some (line) => match line {
                Ok (line) => Ok(Some(line)),
                Err (error) => Err(SourceError::IO(error)),
            }
        }
    }

    pub fn header (&mut self) -> SourceResult<SourceHeader> {
        let line = match self.line()? {
            Some (line) => line,
            None => {
                return Err(SourceError::ExpectedHeader);
            },
        };

        let mut header = line.split(' ');

        let cache_total_lines = match header.next() {
            Some (string) => decimal(string),
            None => Err(SourceError::ExpectedDecimal),
        }?;

        let cache_line_size = match header.next() {
            Some (string) => decimal(string),
            None => Err(SourceError::ExpectedDecimal),
        }?;

        Ok(SourceHeader::new(cache_total_lines, cache_line_size))
    }

    pub fn address (&mut self) -> SourceResult<Option<usize>> {
        match self.line()? {
            None => Ok(None),
            Some (line) => Ok(Some(hexadecimal(line.as_str())?)),
        }
    }
}
