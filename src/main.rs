mod io;
use io::PacmanConfig;
mod pathfinding;

fn main() {
    let pacman = &mut PacmanConfig::from_args();
    pacman.pacmap.find_paths();
    println!("{}", pacman);
}
