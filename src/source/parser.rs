use crate::source::{
    SourceError,
    SourceResult,
};

fn trim_hexadecimal_start (string: &str) -> &str {
    string.trim_start_matches("0x")
}

pub fn decimal (string: &str) -> SourceResult<usize> {
    match usize::from_str_radix(string, 10) {
        Ok (number) => Ok(number),
        Err (_) => Err(SourceError::InvalidDecimal),
    }
}

pub fn hexadecimal (string: &str) -> SourceResult<usize> {
    let string = trim_hexadecimal_start(string);

    match usize::from_str_radix(string, 16) {
        Ok (number) => Ok(number),
        Err (_) => Err(SourceError::InvalidHexadecimal),
    }
}