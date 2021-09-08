#[derive(Debug)]
pub enum SourceError {
    ExpectedHeader,

    InvalidDecimal,
    InvalidHexadecimal,

    ExpectedDecimal,
    ExpectedHexadecimal,

    IO (std::io::Error),
}