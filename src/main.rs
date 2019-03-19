#[macro_use]
extern crate clap;
use clap::App;

extern crate chrono;

mod mode;
mod list;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("long") {
        list::list_long();
    } else {
        list::list();
    }
}
