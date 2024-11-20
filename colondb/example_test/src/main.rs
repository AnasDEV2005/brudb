use colon_db::ColonDB;

fn main() {
    let mut database = ColonDB::find_database("db.txt");
    database.insert_item_into_db("01".to_string(),"name".to_string(), "kak".to_string());
    database.insert_item_into_db("01".to_string(),"age".to_string(), "18".to_string());
    database.insert_item_into_db("01".to_string(),"salary".to_string(), "0".to_string());
    database.insert_row_into_db("02".to_string(),vec!["alan".to_string(),"12".to_string(),"23".to_string()]);
    database.delete_column("salary".to_string());
    database.delete_item("08", "name".to_string());
    database.delete_row("09");
    let newdb = database.select_data(Some(0..4), vec!["name".to_string(),"age".to_string()].into());
    newdb.print_database();
    println!("{}",database.select_item("01", "age").unwrap())

}