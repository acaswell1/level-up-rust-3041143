use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        match self.metadata() {
           Ok(meta) => meta.permissions().readonly(),
           Err(_) => false
        }
    }

    fn is_writeable(&self) -> bool {
        match self.metadata() {
           Ok(meta) => !(meta.permissions().readonly()),
           Err(_) => false
        }
    }

    fn exists(&self) -> bool {
        self.metadata().is_ok()
    }
}

fn main() {
    //
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
