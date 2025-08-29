use std::path::PathBuf;

fn get_basename(p: &PathBuf) -> Option<String> {
    // basename with extension.
    let f_base = p
        .file_name()
        .expect("File has invalid basename.")
        .to_str()
        .expect("Failed to convert OsStr to &str");

    let basename: String = f_base.chars().take_while(|c| *c != '.').collect();

    match basename.len() {
        0 => None,
        _ => Some(basename),
    }
}

pub fn replace_extension(f: &PathBuf, suffix: Option<&str>, extension: &str) -> PathBuf {
    let mut f_dir = f
        .parent()
        .expect("Failed to extract parent dir for file.")
        .to_path_buf();

    let mut f_base = get_basename(&f).unwrap();

    match suffix {
        Some(suffix) => f_base = format!("{}_{}", f_base, suffix),
        None => {}
    }

    f_dir.push(f_base);

    let f_new_extension = f_dir.with_extension(extension);
    return f_new_extension;
}
