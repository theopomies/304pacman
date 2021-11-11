use super::get_args::get_args;
use crate::pacmap::{Cell, PacMap};
use std::{
    fmt::{self, Display, Formatter},
    process,
};

#[derive(Debug)]
pub struct PacmanConfig {
    pacmap: PacMap,
    wall_char: char,
    empty_char: char,
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

    pub fn find_paths(&mut self) {
        self.pacmap.find_paths()
    }
}
