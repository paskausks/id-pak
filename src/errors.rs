//! Errors specific to _id-pak_.

/// Errors encountered when loading and parsing PAK data
#[derive(Debug)]
pub enum IdPakLoadError {
    /// Encountered when the PAK
    /// data fails to load, e.g. the path
    /// is incorrect or there aren't sufficient permissions.
    FileOpenFailure(std::io::Error),

    /// An error was encountered when
    /// attempting to read the file headers
    /// or it's table of contents.
    UpdateFailure,
}
