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

    csv_db.print_data();

    match csv_db.delete_by_id(3) {
        Ok(_) => {}
        Err(e) => println!("Error {}", e),
    }

    let id = 3;
    match csv_db.search_by_id(id) {
        Some(row) => println!("row with id {id} -> {row}"),
        None => println!("item with id {id} not found"),
    }

    Ok(())
}
