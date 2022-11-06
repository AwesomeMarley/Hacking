extern crate rand;

use std::io::stdout;
extern crate crossterm;
use crossterm::{cursor, terminal, ExecutableCommand};

mod readfile;
mod scramblefile;

const ENCRYPTION_LEVEL: i32 = 100000;

fn main() {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    let file =
        readfile::read_file_by_line("D:/Code/VS-CODE/Rust/Hacking/assets/hackedfile.txt").unwrap();

    let file_content = scramblefile::convert_content_to_chars(file);

    let mut scrambled_content = scramblefile::scramble_file(&file_content);

    loop {
        let mut complete = true;
        for line in &scrambled_content {
            for (char, origin_char, _) in line {
                if char != origin_char {
                    complete = false;
                }
                print!("{}", char);
            }
            println!();
        }

        if complete {
            break;
        }

        // let mut stdout = stdout();

        scrambled_content = scramblefile::scramble_file(&scrambled_content);

        std::thread::sleep(std::time::Duration::from_millis(10));

        stdout
            .execute(cursor::MoveUp(scrambled_content.len() as u16))
            .unwrap();
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }
}
