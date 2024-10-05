fn find_python_executable(base_dir: &Path) -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        // Windows-specific search
        let mut possible_paths = vec![
            python_executable.with_file_name("pip.exe"),
            python_executable
                .parent()
                .unwrap()
                .join("Scripts")
                .join("pip.exe"),
        ];
        possible_paths
            .into_iter()
            .find(|path| path.exists())
            .ok_or_else(|| "pip.exe not found".to_string())?
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
