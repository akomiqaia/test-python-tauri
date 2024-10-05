use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use tauri_plugin_shell::ShellExt;

fn install_python_packages(python_executable: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing required Python packages...");
    let pip_executable = if cfg!(target_os = "windows") {
        python_executable.with_file_name("pip.exe")
    } else {
        python_executable.with_file_name("pip3")
    };
    let requirements_path = Path::new("../python_server/requirements.txt");

    if !requirements_path.exists() {
        return Err("requirements.txt not found".into());
    }

    let output = Command::new(&pip_executable)
        .arg("install")
        .arg("-r")
        .arg(requirements_path)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to install packages: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    println!("Packages installed successfully");
    Ok(())
}

fn find_python_executable(base_dir: &Path) -> Option<PathBuf> {
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
    None
}

fn download_and_extract_python(python_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (python_url, is_zip) = if cfg!(target_os = "windows") {
        ("https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3+20230507-x86_64-pc-windows-msvc-shared-install_only.zip", true)
    } else {
        ("https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3+20230507-x86_64-apple-darwin-install_only.tar.gz", false)
    };

    println!("Downloading Python distribution...");
    let response = get(python_url)?;

    println!("Extracting Python distribution...");
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
    let python_dir = Path::new("../python");

    if !Path::new("../python_server/requirements.txt").exists() {
        return Err(
            "requirements.txt file not found. Please ensure it exists in the current directory."
                .to_string(),
        );
    }

    fs::create_dir_all(python_dir)
        .map_err(|e| format!("Failed to create Python directory: {}", e))?;

    let python_executable = match find_python_executable(python_dir) {
        Some(exe) => {
            println!("Python installation found at {:?}", exe);
            exe
        }
        None => {
            println!("Python not found. Installing...");
            download_and_extract_python(python_dir)
                .map_err(|e| format!("Failed to download and extract Python: {}", e))?;
            find_python_executable(python_dir)
                .ok_or_else(|| "Failed to find Python executable after installation. Please check your installation and try again.".to_string())?
        }
    };

    install_python_packages(&python_executable)
        .map_err(|e| format!("Failed to install Python packages: {}", e))?;

    // Read the FastAPI server script from file
    let python_script = fs::read_to_string("../python_server/api.py")
        .map_err(|e| format!("Failed to read FastAPI server script: {}", e))?;

    // Write the Python script to a file in the Python directory
    let script_path = python_dir.join("fastapi_server.py");
    fs::write(&script_path, python_script)
        .map_err(|e| format!("Failed to write FastAPI server script: {}", e))?;

    // Run the Python script
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
            println!("Python server started successfully");
            println!("Result: {:?}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!(
                "Failed to start Python server. Exit code: {:?}",
                output.status.code()
            );
            eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
        }
    });
    Ok(())
}
