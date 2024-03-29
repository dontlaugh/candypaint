# candypaint

Candy coated prompts for the ion shell. Tested on Linux only, for now.

## Installation

Use cargo. Note that ~/.cargo/bin must be on your PATH

```
cargo install --git https://gitlab.redox-os.org/coleman/candypaint.git
```

Then, add the following to **~/.config/ion/initrc**

```sh
# use the "chad" theme as your prompt
fn PROMPT
    let tmp = "/tmp/candypaint"
    echo -n $(candypaint chad) > $tmp
    source $tmp
    echo -n $CANDY
end
```

In this example, candypaint writes a script to `$tmp`. That script exports a 
`CANDY` variable, our dynamically generated prompt. It must be sourced by ion
to interpret the escape characters correctly.

## Overview

This project aims to provide zero-config prompts for ion. Prompt modifications
might get deeper integration into ion in the future, but until then we can use
a dedicated tool to shell out to in our initrc.

## Themes

We have 2 so far.

**chad**

![chad](assets/chad.png)


**darkside**

![darkside](assets/darkside.png)


A theme is simply a function that returns `Option<String>` built from your 
environment (your path, your git status, etc.). 

Got an idea? See **CONTRIBUTING.md** to see how to add a prompt.

