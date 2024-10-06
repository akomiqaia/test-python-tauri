use flate2::read::GzDecoder;
use log;
use reqwest::blocking::get;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

fn install_python_packages(
    app_handle: &tauri::AppHandle,
    python_executable: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing required Python packages...");
    log::info!("Installing required Python packages...");

    let requirements_path = app_handle
        .path()
        .resolve("python_server/requirements.txt", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    log::info!("requirements_path: {:?}", requirements_path);
    if !requirements_path.exists() {
        log::error!("requirements.txt not found in bundled resources");
        return Err("requirements.txt not found in bundled resources".into());
    }

    let output = Command::new(&python_executable)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("-r")
        .arg(&requirements_path)
        .output()?;

    if !output.status.success() {
        log::error!(
            "Failed to install packages: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(format!(
            "Failed to install packages: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    println!("Packages installed successfully");
    log::info!("Packages installed successfully");
    Ok(())
}

fn find_python_executable(base_dir: &Path) -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        // Windows-specific search
        let python_exe = base_dir.join("python").join("python.exe");
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

fn download_and_extract_python(python_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (python_url, is_zip) = if cfg!(target_os = "windows") {
        ("https://github.com/indygreg/python-build-standalone/releases/download/20241002/cpython-3.12.7+20241002-x86_64-pc-windows-msvc-install_only.tar.gz", false)
    } else {
        ("https://github.com/indygreg/python-build-standalone/releases/download/20241002/cpython-3.12.7+20241002-aarch64-apple-darwin-install_only.tar.gz", false)
    };

    println!("Downloading Python distribution...");
    log::info!("Downloading Python distribution...");
    let response = get(python_url)?;

    println!("Extracting Python distribution...");
    log::info!("Extracting Python distribution...");
    if is_zip {
        let mut zip_archive = zip::ZipArchive::new(std::io::Cursor::new(response.bytes()?))?;
        zip_archive.extract(python_dir)?;
    } else {
        let tar_gz = GzDecoder::new(response);
        let mut archive = Archive::new(tar_gz);
        archive.unpack(python_dir)?;
    }

    Ok(())
}

pub fn start_python_server(app_handle: tauri::AppHandle) -> Result<(), String> {
    let python_dir = app_handle
        .path()
        .resolve("python_server", BaseDirectory::Resource)
        .unwrap();

    let python_executable = match find_python_executable(&python_dir) {
        Some(exe) => {
            log::info!("Python installation found at {:?}", exe);
            println!("Python installation found at {:?}", exe);
            exe
        }
        None => {
            log::info!("Python not found. Installing...");
            println!("Python not found. Installing...");
            download_and_extract_python(&python_dir)
                .map_err(|e| format!("Failed to download and extract Python: {}", e))?;
            find_python_executable(&python_dir)
                .ok_or_else(|| "Failed to find Python executable after installation. Please check your installation and try again.".to_string())?
        }
    };

    install_python_packages(&app_handle, &python_executable)
        .map_err(|e| format!("Failed to install Python packages: {}", e))?;

    // Write the Python script to a file in the Python directory
    let script_path = python_dir.join("api.py");

    // Run the Python script
    log::info!("Running Python script...");
    println!("Running Python script...");

    tauri::async_runtime::spawn(async move {
        let output = app_handle
            .shell()
            .command(&python_executable)
            .args([&script_path])
            .output()
            .await
            .unwrap();
        if output.status.success() {
            log::info!("Python server started successfully");
            log::info!("Result: {:?}", String::from_utf8_lossy(&output.stdout));
            println!("Python server started successfully");
            println!("Result: {:?}", String::from_utf8_lossy(&output.stdout));
        } else {
            log::error!(
                "Failed to start Python server. Exit code: {:?}",
                output.status.code()
            );
            eprintln!(
                "Failed to start Python server. Exit code: {:?}",
                output.status.code()
            );
            log::error!("Error output: {}", String::from_utf8_lossy(&output.stderr));
            eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
        }
    });
    Ok(())
}
