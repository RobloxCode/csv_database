mod csv_database;

use csv_database::CsvDatabase;

fn main() -> std::io::Result<()> {
    let mut csv_db = CsvDatabase::new("src/users.csv").unwrap();

    match csv_db.add("") {
        Ok(_) => {}
        Err(e) => println!("Error {}", e),
    }

    match csv_db.add("3,patrick,23,somemail@gmail.com") {
        Ok(_) => {}
        Err(e) => println!("Error {}", e),
    }

    Ok(())
}
