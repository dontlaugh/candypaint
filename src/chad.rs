use super::*;
use clap::{Arg, ArgMatches};

pub fn cmd<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("chad")
        .author("Coleman Emery McFarland")
        .version("1.0.0")
        .about("RIP Chad Butler")
}

pub fn prompt<'a>(_: &ArgMatches<'a>) -> Option<String> {
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