use crate::pathfinding::{Cell, PacMap};
use clap::{clap_app, ErrorKind};
use std::{
    env,
    fmt::{self, Display, Formatter},
    process,
};

#[derive(Debug)]
pub struct PacmanConfig {
    pacmap: PacMap,
    wall_char: char,
    empty_char: char,
}

#[derive(Debug)]
struct Arguments {
    pub map_file: String,
    pub wall_char: char,
    pub empty_char: char,
}

impl Display for PacmanConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s: String = self
            .pacmap
            .map
            .iter()
            .map(|l| {
                l.iter()
                    .map(|c| match c {
                        Cell::EmptySpace { dist: Some(n), .. } => {
                            char::from_digit(*n as u32 % 10, 10).unwrap()
                        }
                        Cell::Ghost => 'F',
                        Cell::Pacman => 'P',
                        Cell::Wall => self.wall_char,
                        _ => self.empty_char,
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        f.write_str(&s)
    }
}

impl PacmanConfig {
    pub fn from_args() -> Self {
        let args = get_args();
        let file_string = std::fs::read_to_string(args.map_file).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(84);
        });
        Self {
            pacmap: PacMap::from(file_string),
            wall_char: args.wall_char,
            empty_char: args.empty_char,
        }
    }
}

fn get_args() -> Arguments {
    let args = match clap_app!(pacman =>
        (@arg file: +required "file describing the board, using the following characters:
        ‘0’ for an empty square,
        ‘1’ for a wall,
        ‘F’ for the ghost’s position,
        ‘P’ for Pacman’s position.")
        (@arg c1: +required "character to display for a wall.")
        (@arg c2: +required "character to display for an empty space.")
    )
    .get_matches_from_safe(env::args())
    {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err.message);
            match err.kind {
                ErrorKind::HelpDisplayed => process::exit(0),
                _ => process::exit(84),
            }
        }
    };

    Arguments {
        map_file: args.value_of("file").unwrap().to_owned(),
        wall_char: args.value_of("c1").unwrap().chars().next().unwrap(),
        empty_char: args.value_of("c2").unwrap().chars().next().unwrap(),
    }
}
