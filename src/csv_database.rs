use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct CsvDatabase {
    path: String,
    file: File,
}

impl CsvDatabase {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)?;

        file.seek(SeekFrom::Start(0))?;

        Ok(Self {
            path: path.to_string(),
            file,
        })
    }

    pub fn print_data(&mut self) {
        let mut contents = String::new();
        let _ = self.file.read_to_string(&mut contents);
        print!("{}", contents);
    }

    pub fn add(&self, row: &str) -> Result<(), std::io::Error> {
        if row.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Empty row"));
        }

        let contents = fs::read_to_string(&self.path)?;
        let new_id = row.chars().nth(0).unwrap_or_default();

        for line in contents.lines() {
            if line.starts_with(&new_id.to_string()) {
                return Err(Error::new(ErrorKind::Other, "id already in file"));
            }
        }

        writeln!(&self.file, "{}", row)?;
        Ok(())
    }

    pub fn delete_by_id(&mut self, id: usize) -> Result<(), std::io::Error> {
        let contents = fs::read_to_string(&self.path)?;

        let del_pos = contents
            .lines()
            .position(|l| l.starts_with(&id.to_string()))
            .ok_or(Error::new(ErrorKind::Other, "No id found"))?;

        let new_contents: String = contents
            .lines()
            .enumerate()
            .filter(|(i, _)| *i != del_pos)
            .map(|(_, l)| format!("{l}\n"))
            .collect();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        file.write_all(new_contents.as_bytes())?;

        Ok(())
    }

    pub fn search_by_id(&self, id: usize) -> Option<String> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(_) => return None,
        };

        contents
            .lines()
            .find(|l| l.starts_with(&id.to_string()))
            .map(|l| l.to_string())
    }
}
