extern crate clap;

use clap::{App, Arg};
use std::env;
use std::fmt::Write;

mod darkside;
mod chad;

fn main() {
    let app = App::new("candypaint")
        .version("0.3.0")
        .about("candy coated prompts for the ion shell")
        .author("Coleman Emery McFarland")
        .subcommand(darkside::cmd())
        .subcommand(chad::cmd());

    let matches = app.get_matches();
    let prompt = match matches.subcommand() {
        ("darkside", Some(matches)) => darkside::prompt(&matches),
        ("chad", Some(matches)) => chad::prompt(&matches),
        _ => chad::prompt(&matches),
    };

    println!(
        "export CANDY = \"{}\"",
        prompt.unwrap_or_else(|| String::from("export CANDY = \"# ${c::reset}\""))
    );
}

pub fn git_info() -> Option<GitInfo> {
    use std::process::Command;
    let mut cmd = Command::new("git");
    cmd.args(&["rev-parse", "--symbolic-full-name", "--abbrev-ref", "HEAD"]);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8(output.stdout).ok()?;
    Some(GitInfo { branch: branch })
}

/// GitInfo holds state related to the current git repo, if present.
#[derive(Debug)]
pub struct GitInfo {
    pub branch: String,
}


fn cycle<T, I: DoubleEndedIterator<Item = T> + Clone>(
    iter: I,
) -> std::iter::Cycle<std::iter::Chain<I, std::iter::Rev<I>>> {
    iter.clone().chain(iter.rev()).cycle()
}
