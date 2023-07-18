use tantivy::{
    directory::MmapDirectory,
};
use std::path::Path;
use tantivy::directory::error::{OpenReadError, OpenWriteError, LockError, DeleteError};
use tantivy::directory::{Directory, FileSlice, WritePtr, DirectoryLock};
use tantivy::directory::Lock;
use tantivy::directory::FileHandle;
use tantivy::directory::WatchHandle;
use tantivy::directory::WatchCallback;
use tantivy::TantivyError;
use std::{io};
use std::sync::{Arc};

#[derive(Debug, Clone)]
pub struct ReadOnlyDirectoryWrapper {
    pub inner: MmapDirectory,
}

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

impl Directory for ReadOnlyDirectoryWrapper {
    fn get_file_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>, OpenReadError> {
        MmapDirectory::get_file_handle(&self.inner, path)
    }

    fn acquire_lock(&self, _lock: &Lock) -> Result<DirectoryLock, LockError> {
        Ok(DirectoryLock::from(Box::new(HasDrop)))
    }

    fn open_read(&self, path: &Path) -> Result<FileSlice, OpenReadError> {
        MmapDirectory::open_read(&self.inner, path)
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        MmapDirectory::delete(&self.inner, path)
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        MmapDirectory::exists(&self.inner, path)
    }

    fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
        MmapDirectory::open_write(&mut &self.inner, path)
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        MmapDirectory::atomic_read(&self.inner, path)
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        MmapDirectory::atomic_write(&mut &self.inner, path, data)
    }

    fn watch(&self, watch_callback: WatchCallback) -> Result<WatchHandle, TantivyError> {
        MmapDirectory::watch(&self.inner, watch_callback)
    }

    fn sync_directory(&self) -> io::Result<()> {
        Ok(())
    }
}