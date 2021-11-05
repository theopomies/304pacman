mod io;
use io::PacmanConfig;
mod pacmap;

fn main() {
    let pacman = &mut PacmanConfig::from_args();
    pacman.pacmap.find_paths();
    println!("{}", pacman);
}
