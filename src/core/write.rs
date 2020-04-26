#[cfg(test)]
mod test {
    use super::{write_csv, replace_column};

    #[test]
    fn test_valid_write_csv() {
        let data = "test";
        let filename = "./output/test.csv";
        let write_info = write_csv(data, filename);
        assert!(write_info.is_ok());
    }

    #[test]
    // ＃[ignore]
    fn test_valid_replace_column() {
        let data = "First Name,Last Name,Age,City,Eyes color,Species
John,Doe,32,Beijing,Blue,Human
Flip,Helm,12,Beijing.Blue,Unknown
Terdos,Bendarian,165,Beijing,Blue,Magic tree
Dominik,Elpos,33,Beijing,Purple,Orc
Brad,Doe,42,Beijing,Blue,Human
Ewan,Grath,51,Beijing,Green,Human".to_string();
        let column = "City";
        let replacement = "Shanghai";
        let res = replace_column(data, column, replacement);
        assert!(res.is_ok());
        println!("{:?}", res.unwrap())
    }
}

use super::*;

/// # Usage:
/// ```
/// use csv_challenge::{load_csv, replace_column, write_csv};
/// use std::path::PathBuf;
/// let filename = PathBuf::from("input/challenge.csv");
/// let csv_data = load_csv(filename).unwrap();
/// let modified_data = replace_column(csv_data, "City", "Shanghai").unwrap();
/// let output_file = write_csv(&modified_data, "output/test.csv");
/// assert!(output_file.is_ok());
///```

pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
    write(csv_data, filename)?;
    Ok(())
}

fn write(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}

/// # Usage:
/// ```
/// use csv_challenge::{load_csv, replace_column};
/// use std::path::PathBuf;
/// let filename = PathBuf::from("input/challenge.csv");
/// let csv_data = load_csv(filename).unwrap();
/// let modified_data = replace_column(csv_data, "City", "Shanghai");
/// assert!(modified_data.is_ok());
///```

pub fn replace_column(data: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = data.lines();
    let headers = lines.next().unwrap();
    let columns:Vec<&str> = headers.split(',').collect();
    let column_number = columns.iter().position(|&e| e == column);
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name dosen't exist in the input file")?
    };
    let mut result = String::with_capacity(data.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');
    for line in lines {
        let mut records:Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }
    Ok(result)
}