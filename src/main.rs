extern crate clap;

use clap::{App, Arg};
use std::env;
use std::fmt::Write;

fn cycle<T, I: DoubleEndedIterator<Item = T> + Clone>(
    iter: I,
) -> std::iter::Cycle<std::iter::Chain<I, std::iter::Rev<I>>> {
    iter.clone().chain(iter.rev()).cycle()
}

mod darkside;

fn main() {
    let app = App::new("candypaint")
        .version("0.3.0")
        .about("candy coated prompts for the ion shell")
        .author("Coleman Emery McFarland")
        .subcommand(darkside::cmd());

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("darkside") {
        darkside::prompt(&matches);
    }

    let prompt = match matches.value_of("theme") {
        Some(theme) => match theme {
            "chad" => chad(),
            "darkside" => darkside(),
            _ => chad(),
        },
        _ => None,
    };

    println!(
        "export CANDY = \"{}\"",
        prompt.unwrap_or_else(|| String::from("export CANDY = \"# ${c::reset}\""))
    );
}

/// chad is our default theme.
fn chad() -> Option<String> {
    let mut ret = String::new();

    if let Ok(user) = env::var("USER") {
        for (c, color) in user.chars().zip(cycle((0xd0..0xde).rev())) {
            write!(&mut ret, "${{c::0x{:X},bold}}{}", color, c).ok()?;
        }
        ret.push_str("${c::0xd7}:")
    }

    if let Ok(path) = env::current_dir() {
        if let Some(pwd) = path.file_name() {
            write!(&mut ret, "${{c::0xd6}}{}", pwd.to_str().unwrap_or("")).ok()?;
        }
    }

    if let Some(git_info) = git_info() {
        write!(
            &mut ret,
            " (${{c::0xb8}}{}${{c::0xd6}}) ${{c::0x05}}# ${{c::reset}}",
            &git_info.branch.trim()
        )
        .ok()?;
    } else {
        ret.push_str(" ${c::0x05}# ${c::reset}");
    }

    Some(ret)
}

/// darkside is scary.
fn darkside() -> Option<String> {
    let mut path = String::new();
    if let Ok(cwd) = env::current_dir() {
        if let Some(val) = cwd.as_path().to_str() {
            path.push_str(val);
        }
    }

    // black -> light grey
    let mut temp = String::new();

    for (c, color) in path.chars().zip(cycle(0xe8..0xfe)) {
        write!(&mut temp, "${{c::0x{:X},bold}}{}", color, c).ok()?;
    }

    if let Some(git_info) = git_info() {
        write!(
            &mut temp,
            " ${{c::0x7c}}<<{}${{c::0x7c}}>> ${{c::reset}}",
            &git_info.branch.trim()
        )
        .ok()?;
    } else {
        temp.push_str(" ${c::0x7c}>> ${c::reset}");
    }
    Some(temp)
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
