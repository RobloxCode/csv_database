use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};

use std::fs;

#[derive(Debug)]
struct CsvDatabase {
    path: String,
    file: File,
}

impl CsvDatabase {
    fn new(path: &str) -> std::io::Result<Self> {
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

    fn print_data(&mut self) {
        let mut contents = String::new();
        let _ = self.file.read_to_string(&mut contents);
        print!("{}", contents);
    }

    fn add(&self, row: &str) -> Result<(), std::io::Error> {
        writeln!(&self.file, "{}", row)?;
        Ok(())
    }

    fn delete_by_id(&mut self, id: usize) -> Result<(), std::io::Error> {
        let contents = fs::read_to_string(&self.path)?;

        let del_pos = contents
            .lines()
            .position(|l| l.starts_with(&id.to_string()))
            .ok_or(Error::new(ErrorKind::Other, "no id found"))?;

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

    fn search_by_id(&self, id: usize) -> Option<String> {
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

fn main() -> std::io::Result<()> {
    let mut csv_db = CsvDatabase::new("src/users.csv").unwrap();
    csv_db.add("3,Patrick,23,patrick@gmail.com")?;

    csv_db.print_data();

    // csv_db.delete_by_id(1)?;
    // csv_db.delete_by_id(4)?;
    csv_db.delete_by_id(4)?;

    csv_db.search_by_id(0);
    if let Some(line) = csv_db.search_by_id(8) {
        println!("line with id 3 is {line}");
    }

    Ok(())
}
