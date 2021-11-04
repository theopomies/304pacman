mod io;
use io::PacmanConfig;
mod pathfinding;

fn main() {
    let pacman = PacmanConfig::from_args();
    println!("{}", pacman);
}
