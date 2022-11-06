use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

pub fn read_file_by_line(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut content = vec![];

    for line in reader.lines() {
        content.push(line?);
    }

    Ok(content)
}