use std::process::Command;


pub fn git_init(current_directory : &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .arg("init")
        .current_dir(current_directory)
        .status()?;

    Ok(())
        
}
