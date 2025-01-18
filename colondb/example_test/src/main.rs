use colon_db::ColonDB;

fn main() {
    let mut database = ColonDB::find_database("test.txt");
    // database.set_header("id".to_string(), vec!["name".to_string(),"age".to_string(),"salary".to_string()]);
    database.append_row_to_db("21".to_string(),vec!["alaaaan".to_string(),"12".to_string(),"23000".to_string()]);
    database.append_row_to_db("22".to_string(),vec!["palaaaan".to_string(),"12".to_string(),"2300".to_string()]);
    database.append_row_to_db("23".to_string(),vec!["a".to_string(),"12".to_string(),"3000".to_string()]);
    database.append_row_to_db("24".to_string(),vec!["agggaaan".to_string(),"13".to_string(),"1300".to_string()]);
    database.print_database();
    database.insert_item_into_db("21".to_string(),"name".to_string(), "kak".to_string());
    database.insert_item_into_db("21".to_string(),"age".to_string(), "18".to_string());
    database.insert_item_into_db("21".to_string(),"salary".to_string(), "0".to_string());
    database.delete_row("22");
    let newdb = database.select_data(Some(4..17), vec!["name".to_string(), "salary".to_string()].into());
    let mut db = newdb.unwrap();
    db.append_column("status".to_string(), "citizen".to_string());
    db.print_database();

    database.print_database();
    println!("{}",database.get_item("01", "age").unwrap())

}
