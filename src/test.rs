fn read_file(path: &PathBuf) -> Result<Config, some_err> {
    let file = fs::read_to_string(&path)?;
    Ok(file)
}
