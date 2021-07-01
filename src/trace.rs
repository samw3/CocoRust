use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

pub struct Trace {
    dir: PathBuf,
    file: Option<File>,
}

impl Trace {
    pub fn new(dir: PathBuf) -> Self {
        Trace {
            dir,
            file: None,
        }
    }

    fn ensure_file(&mut self) {
        match self.file {
            None => {
                let mut path = PathBuf::from(&self.dir);
                path.push("trace.txt");
                let file = File::create(path);
                if let Err(_) = file {
                    panic!("Could not open file {}", &self.dir.display());
                }
                self.file = file.ok();
            }
            _ => {}
        }
    }

    pub fn format_string(&self, string: String, width: i32) -> String {
        let mut output = String::new();
        let size: i32 = string.len() as i32;

        if width >= 0 {
            for _ in 0..(width - size) {
                output.push(' ');
            }
            output.push_str(&string);
        } else {
            output.push_str(&string);
            for _ in width..(-size) {
                output.push(' ');
            }
        }
        output
    }

    pub fn write(&mut self, string: String) {
        self.ensure_file();
        write!(self.file.as_ref().unwrap(), "{}", string).ok();
    }

    // writes a string with a minimum length of |w| characters
    pub fn write_wide(&mut self, string: String, width: i32) {
        self.write(self.format_string(string, width));
    }

    pub fn write_line(&mut self, string: String) {
        self.ensure_file();
        writeln!(self.file.as_ref().unwrap(), "{}", string).ok();
    }

    pub fn write_line_wide(&mut self, string: String, width: i32) {
        self.write_line(self.format_string(string, width));
    }
}
