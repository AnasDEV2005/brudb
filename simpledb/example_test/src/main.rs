use simple_db::SimpleDB;

fn main() {
    let mut database = SimpleDB::find_database("teeeest.txt");
    let mut db = database.unwrap(); // this or handle the error 
    db.insert_into_db("ID".to_string(), "Value".to_string());

}
