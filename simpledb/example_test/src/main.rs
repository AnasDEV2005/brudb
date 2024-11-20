use simple_db::SimpleDB;

fn main() {
    let mut database = SimpleDB::find_database("db2.txt");
    database.insert_into_db("ID".to_string(), "Value".to_string());
    database.insert_into_db("ray".to_string(), "123".to_string());
    database.insert_into_db("jay".to_string(), "456".to_string());
    database.insert_into_db("kay".to_string(), "789".to_string());
}