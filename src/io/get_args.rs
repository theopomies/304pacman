#[derive(Debug)]
pub struct Arguments {
    pub map_file: String,
    pub wall_char: char,
    pub empty_char: char,
}

const HELP_MESSAGE: &str = r#"
USAGE
    ./304pacman file c1 c2

ARGUMENTS
    file    file describing the board, using the following characters:
                ‘0’ for an empty square,
                ‘1’ for a wall,
                ‘F’ for the ghost’s position,
                ‘P’ for Pacman’s position.
    c1      character to display for a wall.
    c2      character to display for an empty space."#;

pub fn get_args() -> Arguments {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match &args[..] {
        [file, c1, c2] if c1.len() == 1 && c2.len() == 1 && c1 != c2 => Arguments {
            map_file: file.to_owned(),
            wall_char: c1.chars().next().unwrap(),
            empty_char: c2.chars().next().unwrap(),
        },
        [arg, ..] if arg == "-h" || arg == "--help" => {
            println!("{}", HELP_MESSAGE);
            std::process::exit(0);
        }
        _ => {
            eprintln!("{}", HELP_MESSAGE);
            std::process::exit(84);
        }
    }
}
