
use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use std::fs;

#[test]
/// Testing pan-sv
/// Parameters.
///     --gfa
///     -o test1
///
fn main_solo() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pan-sv")?;
    fs::create_dir_all("./data/example_data/test1")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa")
        .arg("-o")
        .arg("./data/example_data/test1/test1");

    cmd.assert().success();
    let foo: String = fs::read_to_string("data/example_data/test1/test1.bubble.stats").unwrap();
    assert_eq!(foo.contains("2	11	26"), true);

    let path = "data/example_data/test1";
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();


    Ok(())
}




