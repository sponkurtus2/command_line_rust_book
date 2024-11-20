use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

// This will be a custom type of error
type TestResult = Result<(), Box<dyn std::error::Error>>;

// CONST -> An unchangeable value (the common case)
const FILE1: &str = "tests/expected/hello1.txt";

//STATIC -> A possibly mutable variable with 'static lifetime
static FILE2: &str = "tests/expected/hello2.txt";

const FILE3: &str = "tests/expected/hello1.n.txt";
const FILE4: &str = "tests/expected/hello2.n.txt";

// To run this test, we can run 'cargo test dies', and Cargo will run all
// The test that include the string 'dies'
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd: Command = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd: Command = Command::cargo_bin("echor")?;
    cmd.arg("Hello").assert().success();
    Ok(())
}

//#[test]
//fn hello1() -> TestResult {
//    let outfile: &str = "tests/expected/hello1.txt";
//    let expected: String = fs::read_to_string(outfile)?;
//
//    let mut cmd: Command = Command::cargo_bin("echor")?;
//    cmd.arg("Hello there").assert().success().stdout(expected);
//    Ok(())
//}
//
//#[test]
//fn hello2() -> TestResult {
//    let outfile: &str = "tests/expected/hello2.txt";
//    let expected: String = fs::read_to_string(outfile)?;
//
//    let mut cmd: Command = Command::cargo_bin("echor")?;
//    cmd.args(vec!["Hello", "there"])
//        .assert()
//        .success()
//        .stdout(expected);
//
//    Ok(())
//}

// Here, we use &[&str] for args, which is a slice
// Slice is like a vector, but they cannot be resized after creation
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected: String = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], FILE1)
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], FILE2)
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello  there", "-n"], FILE3)
}

#[test]
fn hello4() -> TestResult {
    run(&["-n", "Hello", "there"], FILE4)
}
