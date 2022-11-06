use rand::Rng;

use crate::{ENCRYPTION_LEVEL};

const CHARS: &str = "\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789)(*&^%$#@!~\
`-_=+[{]}\\|;:'\",<.>/?\
            ";

pub fn scramble_file(content: &Vec<Vec<(char, char, i32)>>) -> Vec<Vec<(char, char, i32)>> {
    let mut scrambled_content = vec![];

    for line in content {
        let mut scrambled_line = vec![];

        for (char, origin_char, decrypt) in line {
            if *decrypt > 0 {
                scrambled_line.push((scramble_char(*char), *origin_char, decrypt - 1));
            } else {
                scrambled_line.push((*origin_char, *origin_char, 0));
            }
        }

        scrambled_content.push(scrambled_line);
    }

    scrambled_content
}

fn scramble_char(c: char) -> char {
    if c == ' ' && c == '\t' {
        return c;
    }

    let scrambled_char = CHARS
        .chars()
        .nth(rand::thread_rng().gen_range(0..CHARS.len()))
        .unwrap();

    scrambled_char
}

pub fn convert_content_to_chars(content: Vec<String>) -> Vec<Vec<(char, char, i32)>> {
    let mut converted_content = vec![];

    for line in content {
        let mut converted_line = vec![];
        let mut chars = line.chars();

        while let Some(c) = chars.next() {
            let rand_start: i32 = rand::thread_rng().gen_range(50..ENCRYPTION_LEVEL) as i32;
            converted_line.push((c, c, rand_start));
        }

        converted_content.push(converted_line);
    }

    converted_content
}
