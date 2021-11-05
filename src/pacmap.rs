use std::process;

#[derive(Debug)]
pub struct PacMap {
    pub map: Map,
    ghost_pos: Position,
    stack: Vec<Position>,
}

type Map = Vec<Vec<Cell>>;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub enum Cell {
    Wall,
    EmptySpace { dist: Option<u8> },
    Ghost,
    Pacman,
}

impl PacMap {
    pub fn find_paths(&mut self) {
        if self.visit_neighbours(self.ghost_pos) {
            return;
        }
        while !self.stack.is_empty() {
            let next_position = self.stack.remove(0);
            if self.visit_neighbours(next_position) {
                return;
            }
        }
    }

    fn visit_neighbours(&mut self, from_pos: Position) -> bool {
        let Position { x, y } = from_pos;
        let current_distance = if let Cell::Ghost = self.map[y][x] {
            0
        } else if let Cell::EmptySpace { dist, .. } = self.map[y][x] {
            dist.unwrap()
        } else {
            return false;
        };

        from_pos
            .north()
            .map(|pos| self.visit_neighbour(pos, current_distance))
            .unwrap_or(false)
            || from_pos
                .east(self.map[0].len())
                .map(|pos| self.visit_neighbour(pos, current_distance))
                .unwrap_or(false)
            || from_pos
                .south(self.map.len())
                .map(|pos| self.visit_neighbour(pos, current_distance))
                .unwrap_or(false)
            || from_pos
                .west()
                .map(|pos| self.visit_neighbour(pos, current_distance))
                .unwrap_or(false)
    }

    fn visit_neighbour(&mut self, visited_pos: Position, current_distance: u8) -> bool {
        let Position { x, y } = visited_pos;
        match &mut self.map[y][x] {
            Cell::Pacman => true,
            Cell::EmptySpace { dist, .. } => {
                if let Some(dist) = dist {
                    if *dist > current_distance + 1 {
                        *dist = current_distance + 1;
                        self.stack.push(visited_pos);
                    }
                } else {
                    *dist = Some(current_distance + 1);
                    self.stack.push(visited_pos);
                }
                false
            }
            _ => false,
        }
    }
}

impl Position {
    pub fn north(&self) -> Option<Position> {
        self.y.checked_sub(1).map(|y| Position { y, ..*self })
    }
    pub fn east(&self, limit: usize) -> Option<Position> {
        let &Position { x, y } = self;
        x.checked_add(1)
            .map(|x| {
                if x >= limit {
                    None
                } else {
                    Some(Position { x, y })
                }
            })
            .flatten()
    }
    pub fn south(&self, limit: usize) -> Option<Position> {
        let &Position { x, y } = self;
        y.checked_add(1)
            .map(|y| {
                if y >= limit {
                    None
                } else {
                    Some(Position { x, y })
                }
            })
            .flatten()
    }
    pub fn west(&self) -> Option<Position> {
        self.x.checked_sub(1).map(|x| Position { x, ..*self })
    }
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
                            pacman_pos = Some(Position { x, y });
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
                            ghost_pos = Some(Position { x, y });
                            Cell::Ghost
                        }
                        '0' => Cell::EmptySpace { dist: None },
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
            ghost_pos: ghost_pos.unwrap(),
            stack: vec![],
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
    len.is_some() && len.unwrap() > 0
}
