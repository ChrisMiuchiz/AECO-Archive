#[derive(Debug)]
pub enum ArchiveError {
    HEDReadError,
    HEDCreateError,
    HEDWriteError,
    HEDFormatError,
    DATReadError,
    DATCreateError,
    DATWriteError,
    NamesFormatError,
    OffsetError,
    LengthError,
    UnpackError,
    FileNotPresentError,
}
