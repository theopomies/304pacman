use std::process;

#[derive(Debug)]
pub struct PacMap {
    pub map: Map,
    pacman_pos: Position,
    stack: Vec<Cell>,
}

type Map = Vec<Vec<Cell>>;

#[derive(Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
pub enum Cell {
    Wall,
    EmptySpace {
        weight: u8,
        dist: Option<u8>,
        position: Position,
    },
    Ghost,
    Pacman,
}

impl From<String> for PacMap {
    fn from(input: String) -> Self {
        let mut pacman_pos = None;
        let mut ghost_pos = None;
        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '1' => Cell::Wall,
                        'P' => {
                            if let Some(Position { x: x1, y: y1 }) = pacman_pos {
                                eprintln!(
                                    "Two pacman found. First one ({}, {}), second one ({},{})",
                                    x1, y1, x, y
                                );
                                process::exit(84)
                            }
                            pacman_pos = Some(Position {
                                x: x as u8,
                                y: y as u8,
                            });
                            Cell::Pacman
                        }
                        'F' => {
                            if let Some(Position { x: x1, y: y1 }) = ghost_pos {
                                eprintln!(
                                    "Two ghosts found. First one ({}, {}), second one ({},{})",
                                    x1, y1, x, y
                                );
                                process::exit(84)
                            }
                            ghost_pos = Some(Position {
                                x: x as u8,
                                y: y as u8,
                            });
                            Cell::Ghost
                        }
                        '0' => Cell::EmptySpace {
                            weight: 1,
                            dist: Some(3),
                            position: Position {
                                x: x as u8,
                                y: y as u8,
                            },
                        },
                        _ => {
                            eprintln!("Invalid cell. ({}, {})", x, y);
                            process::exit(84);
                        }
                    })
                    .collect()
            })
            .collect();
        if pacman_pos.is_none() {
            eprintln!("No pacman in the map.");
            process::exit(84)
        }
        if ghost_pos.is_none() {
            eprintln!("No ghost in the map.");
            process::exit(84)
        }
        if !is_valid_map(&map) {
            eprintln!("Map's sides don't have consistent lengths");
            process::exit(84)
        }
        Self {
            map,
            stack: vec![],
            pacman_pos: pacman_pos.unwrap(),
        }
    }
}

fn is_valid_map(map: &Map) -> bool {
    let mut len = None;

    for v in map {
        if let Some(len) = len {
            if v.len() != len {
                return false;
            }
        } else {
            len = Some(v.len());
        }
    }
    len.is_some()
}
