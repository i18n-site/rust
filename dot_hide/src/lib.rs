#[cfg(feature = "walkdir")]
use walkdir::DirEntry;

#[cfg(feature = "walkdir")]
pub fn is(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with('.'))
    .unwrap_or(false)
}

#[cfg(feature = "walkdir")]
pub fn not(entry: &DirEntry) -> bool {
  !is(entry)
}
