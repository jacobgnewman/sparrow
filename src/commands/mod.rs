// register modules
mod main;
mod music;
mod xkcd;

use crate::structs::Command;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(xkcd::commands())
        .chain(music::commands())
        .collect()
}
