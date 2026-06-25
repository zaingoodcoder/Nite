use std::process::Command;

pub fn create_py_venv(current_directory:&str) -> Result<(), std::io::Error>{

    Command::new("python")
    .arg("-m")
    .arg("venv")
    .arg("venv")
    .current_dir(current_directory)
    .status()?;

    Ok(())
    
}