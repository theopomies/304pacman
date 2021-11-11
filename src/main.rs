mod io;
use io::pacman_config::PacmanConfig;
mod pacmap;

fn main() {
    let pacman = &mut PacmanConfig::from_args();
    pacman.find_paths();
    println!("{}", pacman);
}
