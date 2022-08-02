
use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use std::fs;

#[test]
/// Testing pan-sv
/// Parameters.
///     --gfa
///     -o test4
///
fn main_solo() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pan-sv")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa")
        .arg("-o")
        .arg("./data/example_data/test1");

    cmd.assert().success();
    let foo: String = fs::read_to_string("data/example_data/test1.bubble.txt").unwrap();
    assert_eq!(foo.contains("1	{2}	{0}"), true);

    let path = "data/example_data";
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();


    Ok(())
}




