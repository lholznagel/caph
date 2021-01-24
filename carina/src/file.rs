use cachem_utils::CachemError;
use std::io::Cursor;

#[cfg(not(test))]
use tokio::fs::OpenOptions;
#[cfg(not(test))]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Wraps [`tokio::fs::File`] type for easier testability
///
/// When compiled as a test build, all filesystem based implementations
/// are replaced with non filesystem implementations, while mainining
/// the same interface.
///
/// When calling [`FileUtils::write`] it will not write directly in the file, instead
/// it will write in an internal buffer. Only on when [`FileUtils::save`] is
/// called the whole buffer will be written and afterwards cleared
pub struct FileUtils;

impl FileUtils {
    /// Opens the given path with read and write permissions
    #[cfg(not(test))]
    pub async fn open(path: &str) -> Result<Option<Cursor<Vec<u8>>>, CachemError> {
        let file_path = format!(
            "db/{}",
            path
        );
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)
            .await?;

        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;

        if content.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(Cursor::new(content)))
        }
    }

    /// Writes the internal buffer to the file, clears the buffer and flushes
    #[cfg(not(test))]
    pub async fn save(path: &str, data: Cursor<Vec<u8>>) -> Result<(), CachemError> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .await?;
        file.write_all(&data.into_inner()).await?;
        file.flush().await?;
        file.sync_all().await?;
        Ok(())
    }

    /// Mocks the opening of a file and returns an empty [`FileUtils`].
    /// If needed the test can fill the underlying file cursor.
    #[cfg(test)]
    pub async fn open(_: &str) -> Result<Option<Cursor<Vec<u8>>>, CachemError> {
        Ok(None)
    }

    /// Mocks the saving of a file
    #[cfg(test)]
    pub async fn save(_: &str, _: Cursor<Vec<u8>>) -> Result<(), CachemError> {
        Ok(())
    }
}
