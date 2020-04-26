mod opt;
mod err;
mod core;

pub use self::opt::Opt;
pub use self::core:: {
    read::{load_csv},
    write::{replace_column, write_csv}
};

// ! Usage:
// !```
// ! use csv_challenge::{
// !     Opt,
// !     load_csv,
// !     write_csv,
// !     replace_column,
// ! };
// ! use std::path::PathBuf;
// !
// ! let filename = PathBuf::from("./input/challenge.csv");
// ! let csv_data = load_csv(filename).unwrap();
// ! assert!(csv_data.is_ok());
// ! let modified_data = replace_column(csv_data, "City", "Shanghai").unwrap();
// ! assert!(modified_data.is_ok());
// ! let output_file = write_csv(&modified_data, "output/test.csv");
// ! assert!(output_file.is_ok());
// !
// ! ```
