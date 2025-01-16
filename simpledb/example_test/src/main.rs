use simple_db::SimpleDB;

fn main() {
    let mut database = SimpleDB::find_database("db3.txt");
    let mut db = database.unwrap(); // this or handle the error 
    db.insert_into_db("ID".to_string(), "Value".to_string());
    db.insert_into_db("ray".to_string(), "123".to_string());
    db.insert_into_db("jay".to_string(), "456".to_string());
    db.insert_into_db("kay".to_string(), "789".to_string());

    db.get_value_from_db("ray");
    db.get_value_from_db("zay");

    db.delete_from_db("zay");

    db.sort_by_key();
    db.print_db();
    db.sort_by_value();
    db.print_db();

}
