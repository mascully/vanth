use std::io::Write;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tempfile::tempdir;
use vanth::{ContentHash, Vanth, hash as vanth_hash};

fn run_vanth(args: &[&str], input: Option<&str>) -> (String, String, i32) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_vanth_cli"));
    cmd.args(args);
    if let Some(inp) = input {
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(inp.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        (
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
            output.status.code().unwrap(),
        )
    } else {
        let output = cmd.output().unwrap();
        (
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
            output.status.code().unwrap(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Vanth)]
struct Foo {
    inner: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
struct Bar {
    inner: f64,
}

/// Create a value in the database with the `write` command, gets its hash from the CLI stdout output, and retrieve it
/// again with the `get` command.
#[test]
fn test_write_get() {
    let tempdir = tempdir().unwrap();
    let db_path = tempdir.path().join("test.sqlite").to_str().unwrap().to_string();

    let foo = Foo { inner: 6 };

    let (stdout, stderr, exit) = run_vanth(
        &[
            "write",
            "--db",
            &db_path,
            "--ty",
            &Foo::ty().to_string(),
            "--value",
            &serde_json::to_string(&foo).unwrap(),
        ],
        None,
    );
    if exit != 0 {
        panic!("{}", stderr);
    }
    let hash = stdout.trim();
    println!("x{}x", hash);

    let (stdout, stderr, exit) = run_vanth(&["get", "--db", &db_path, "--ty", &Foo::ty().to_string(), &hash], None);
    if exit != 0 {
        panic!("{}", stderr);
    }
    let recovered_foo = serde_json::from_str(&stdout).unwrap();
    assert_eq!(foo, recovered_foo);
}
