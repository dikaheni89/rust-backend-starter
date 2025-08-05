//! External File System Access (dummy/adapter)

pub fn read_external_file(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

pub fn write_external_file(path: &str, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)
}
