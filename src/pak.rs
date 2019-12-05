use crate::errors::IdPakLoadError;
use crate::fileentry::{IdPakFileEntry, FILE_ENTRY_SIZE};
use crate::header::{IdPakHeader, HEADER_SIZE};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub type IdPakLoadResult<T> = Result<T, IdPakLoadError>;
pub type IdPakFile = IdPak<File>;

pub trait IdPakReader {
    fn get_file_bytes(&self, path: &str) -> [u8];
    fn get_file(&self, path: &str) -> File;
}

/// Representation of an Id PAK file
pub struct IdPak<R: Read + Seek> {
    /// Buffered PAK data reader
    reader: BufReader<R>,

    /// Header information
    header: IdPakHeader,

    /// File index
    files: BTreeMap<String, IdPakFileEntry>,
}

impl IdPak<File> {
    /// Load PAK data from a file
    pub fn from_path<P: AsRef<Path>>(path: P) -> IdPakLoadResult<IdPakFile> {
        let file: File = match File::open(&path) {
            Ok(f) => f,
            Err(why) => return Err(IdPakLoadError::FileOpenFailure(why)),
        };

        let mut pak: IdPak<File> = IdPak {
            reader: BufReader::new(file),
            header: IdPakHeader::default(),
            files: BTreeMap::new(),
        };

        pak.update()?;

        Ok(pak)
    }
}

impl<R> IdPak<R>
where
    R: Read + Seek,
{
    /// Load PAK data from a source which implements the
    /// `std::io::Read` and `std::io::Seek` traits.
    pub fn new(source: R) -> IdPakLoadResult<IdPak<R>> {
        let mut pak: IdPak<R> = IdPak {
            reader: BufReader::new(source),
            header: IdPakHeader::default(),
            files: BTreeMap::new(),
        };

        pak.update()?;

        Ok(pak)
    }

    /// Returns the amount of files the PAK contains.
    pub fn get_file_count(&self) -> usize {
        self.files.len()
    }

    /// Read the header and file entries from the loaded PAK data.
    fn update(&mut self) -> IdPakLoadResult<()> {
        self.files = BTreeMap::new();

        self.update_header()?;
        self.update_file_table()?;

        Ok(())
    }

    /// Read the header from the PAK data.
    fn update_header(&mut self) -> IdPakLoadResult<()> {
        let mut buffer = [0u8; HEADER_SIZE as usize];
        match self.reader.seek(SeekFrom::Start(0)) {
            Ok(_) => (),
            Err(_) => return Err(IdPakLoadError::UpdateFailure),
        };

        match self.reader.read_exact(buffer.as_mut()) {
            Ok(_) => (),
            Err(_) => return Err(IdPakLoadError::UpdateFailure),
        };

        self.header = IdPakHeader::try_from(&buffer[..]).unwrap();

        Ok(())
    }

    /// Read the file entries from the PAK data.
    fn update_file_table(&mut self) -> IdPakLoadResult<()> {
        match self.reader.seek(SeekFrom::Start(self.header.offset.into())) {
            Ok(_) => (),
            Err(_) => return Err(IdPakLoadError::UpdateFailure),
        };

        for _ in 0..(self.header.size / FILE_ENTRY_SIZE) {
            let mut buffer: [u8; FILE_ENTRY_SIZE as usize] = [0u8; FILE_ENTRY_SIZE as usize];
            match self.reader.read_exact(buffer.as_mut()) {
                Ok(_) => (),
                Err(_) => return Err(IdPakLoadError::UpdateFailure),
            };

            let file_entry = match IdPakFileEntry::try_from(&buffer[..]) {
                Ok(entry) => entry,
                Err(_) => return Err(IdPakLoadError::UpdateFailure),
            };

            self.files.insert(file_entry.get_name(), file_entry);
        }

        Ok(())
    }
}

/// Open a PAK file from path and read it's
/// contents.
///
/// # Example
///
/// ```no_run
/// extern crate id_pak;
/// use id_pak::errors::IdPakLoadError;
///
/// fn main() -> Result<(), IdPakLoadError> {
///     let pak = id_pak::open("some.pak")?;
///
///     Ok(())
/// }
/// ```
pub fn open<P: AsRef<Path>>(path: P) -> IdPakLoadResult<IdPakFile> {
    IdPak::from_path(path)
}
