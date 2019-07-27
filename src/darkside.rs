use super::*;
use clap::{Arg, ArgMatches};

pub fn cmd<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("darkside")
        .author("The Ion Shell Authors")
        .version("1.0.0")
        .about("A scary prompt")
        .arg(
            Arg::with_name("max-path-chars")
                .help("limit the width of the path printed")
                .takes_value(true),
        )
}

pub fn prompt<'a>(matches: &ArgMatches<'a>) -> Option<String> {
    let limit: Option<usize> = matches
        .value_of("max-path-chars")
        .and_then(|val| usize::from_str_radix(val, 10).ok());

    let mut path = String::new();
    if let Ok(cwd) = env::current_dir() {
        if let Some(val) = cwd.as_path().to_str() {
            if let Some(n) = limit {
                path.push_str(&val.chars().take(n).clone().collect::<String>());
            } else {
                path.push_str(val);
            }
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

fn cycle<T, I: DoubleEndedIterator<Item = T> + Clone>(
    iter: I,
) -> std::iter::Cycle<std::iter::Chain<I, std::iter::Rev<I>>> {
    iter.clone().chain(iter.rev()).cycle()
}
