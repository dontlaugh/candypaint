# Contributing

Contributions of prompts are welcome.

Add a new file **src/myprompt.rs** that matches the name of your prompt with the 
following contents

```rust
use super::*;
use clap::{Arg, ArgMatches};

pub fn cmd<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("myprompt")
        .author("Jane Doe")
        .version("4.2.0")
        .about("Jane's cool prompt");
}

pub fn prompt<'a>(matches: &ArgMatches<'a>) -> Option<String> {
    let mut temp = String::new();
    write!(&mut temp, "COOL >>>");
    Some(temp)
}
```

Then add your prompt as a subcommand to the main application in **main.rs**

```rust 
    // add your subcommand here...
    let app = App::new("candypaint")
        // ...
        .subcommand(myprompt::cmd());


    // and invoke it here ...
    let prompt = match matches.subcommand() {
        ("myprompt", Some(matches)) => myprompt::prompt(&matches),
        // others ...
    };
```


## Constraints

Your prompt should not panic. If it reaches an error condition, return `None`
and we will provide a default.

Adding pure-Rust dependencies to Cargo.toml is okay. Adding native shared
library dependencies is _probably_ okay.

