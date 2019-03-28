#[macro_use]
extern crate structopt;

extern crate chrono;

mod mode;
mod list;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Flags {
    #[structopt(short = "l", long = "long")]
    long: bool,
    #[structopt(parse(from_os_str), default_value = ".")]
    input: PathBuf,
}

fn main() {
    let flags = Flags::from_args();

    if (flags.long == true) {
        list::list_long(flags.input);
    } else {
        list::list(flags.input);

    }
}
