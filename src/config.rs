use std::fs;
use toml::Table;


pub fn get_table(filename: &str) -> Table {
    let conftxt = fs::read_to_string(filename).expect(
        "Can't read file, uwunix.toml doesnt exist?"
    );

    let conftable = conftxt.parse::<Table>().unwrap();

    conftable
}
