#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let filename = PathBuf::from("input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
}

use super::*;

/// # Usage:
/// ```
/// use csv_challenge::{load_csv};
/// use std::path::PathBuf;
/// let filename = PathBuf::from("input/challenge.csv");
/// let csv_data = load_csv(filename);
/// assert!(csv_data.is_ok());
///```

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer).unwrap();
    if buffer.is_empty() {
        return Err("input file missing")?
    }
    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

