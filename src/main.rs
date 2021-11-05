mod io;
use io::PacmanConfig;
mod pacmap;

fn main() {
    let pacman = &mut PacmanConfig::from_args();
    pacman.find_paths();
    println!("{}", pacman);
}
