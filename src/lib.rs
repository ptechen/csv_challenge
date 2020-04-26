mod opt;
mod err;

mod core;

pub use self::opt::Opt;
pub use self::core:: {
    read::{load_csv},
    write::{replace_column, write_csv}
};