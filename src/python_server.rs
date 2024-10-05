fn find_python_executable(base_dir: &Path) -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        // Windows-specific search
        let python_exe = base_dir.join("python.exe");
        if python_exe.exists() {
            return Some(python_exe);
        }
    } else {
        // macOS-specific search
        for entry in fs::read_dir(base_dir).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() && path.file_name()?.to_str()?.starts_with("python") {
                let executable = path.join("bin").join("python3");
                if executable.exists() {
                    return Some(executable);
                }
            }
        }
    }
    None
}
