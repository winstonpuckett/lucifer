use std::{fs, io::Write, path::Path};

use crate::{CommandResult, ExitCode};

pub fn write_file(output_path: &String, json_string: String) -> CommandResult<()> {
    let path = Path::new(output_path);

    create_path(path, output_path)?;

    let path_string = if output_path.ends_with(".json") {
        output_path.to_owned()
    } else {
        String::from(format!("{}/{}", output_path, "results.json"))
    };

    let file = std::fs::File::create(&path_string);

    if file.is_err() {
        return Err((ExitCode::UserError, Some(format!("Could not create the file {:?} to store the test results. Most likely the user does not have write permissions.", path_string))));
    }

    let create_result = file
        .unwrap()
        .write_all(json_string.as_bytes());

    if create_result.is_err() {
        return Err((ExitCode::UserError, Some(format!("Could not create the file {:?} to store the test results. Most likely the user does not have write permissions.", path_string))));
    }

    Ok(())
}

fn create_path(path: &Path, output_path: &String) -> CommandResult<()> {
    let path_exists = path.try_exists();

    if path_exists.is_ok() && path_exists.unwrap() == true {
        // The path already exists.
        return Ok(());
    }

    if !output_path.ends_with(".json") {
        return create_all_directories(path);
    }

    // This must be a request for a json file.
    let parent = path.parent();
    if parent.is_none() {
        // The path is the current directory and
        // the file can be created.
        return Ok(());
    }

    create_all_directories(parent.unwrap())
}

fn create_all_directories(path: &Path) -> CommandResult<()> {
    let r = fs::create_dir_all(path);

    if r.is_ok() {
        Ok(())
    } else {
        Err((ExitCode::UserError, Some(format!("Could not create the path {:?} to store the test results. Most likely the user does not have write permissions.", path))))
    }
}

// TODO: Figure out how to test for folders with incorrect permissions easily.