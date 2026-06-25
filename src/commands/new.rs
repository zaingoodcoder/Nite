use std::fs;
use crate::commands::git::git_init;
use crate::commands::python::create_py_venv;
use crate::commands::constants::*;




pub fn create_project(project_name: &str) -> Result<() , std::io::Error> {

    create_folder(project_name)?;
    create_readme(project_name)?;
    create_gitignore(project_name)?;
    create_main_py(project_name)?;

    git_init(project_name)?;
    create_py_venv(project_name)?;


    Ok(())
}

fn create_folder(project_name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(project_name)?;

    Ok(())
}

fn create_readme(project_name: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", project_name , README_NAME);

    fs::write(path, README_CONTENT)?;

    Ok(())
}

fn create_gitignore(project_name: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", project_name , GITIGNORE_NAME);

    fs::write(path, GITIGNORE_CONTENT)?;

    Ok(())
}


fn create_main_py(project_name: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", project_name , MAIN_FILE_NAME);

    fs::write(path, MAIN_FILE_CONTENT)?;


    Ok(())
}