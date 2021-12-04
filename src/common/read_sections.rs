use std::io::{BufRead, BufReader, Read};

// read_sections returns an iterator over the sections (separated by a blank line) in r.
pub fn read_sections<R: Read>(r: R) -> ReadSections<BufReader<R>> {
    ReadSections {
        reader: BufReader::new(r),
        done: false,
    }
}

pub struct ReadSections<R> {
    reader: R,
    done: bool,
}

impl<R: BufRead> Iterator for ReadSections<R> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let mut section = vec![];
            loop {
                let mut line = String::new();
                match self.reader.read_line(&mut line).unwrap() {
                    0 => {
                        self.done = true;
                        match section.len() {
                            0 => return None,
                            _ => return Some(section),
                        };
                    }
                    1 => return Some(section),
                    _ => section.push(line.trim().to_string()),
                };
            }
        }
    }
}
