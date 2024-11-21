
# colonDB | simpleDB with multiple columns support (database file store)

Checkout main folder for simple key value store.
Make sure you clone this repo inside the directory you write in the cargo.toml's ``[dependencies]  simple_db = ...``
May still contain issues, havent tested enough.

---
### usage

I suggest typing in the first row, which will be used as headers to find stuff based on column headers, in the .txt file before dealing with data.
<p>
add to Cargo.toml <br>
(edit the path as suits you)
</p>

```toml
[package]
name = "libtest"
version = "0.1.0"
edition = "2021"

[dependencies]
colon_db = { path = "../../colondb" }
```
</p>

<p>
use in main.rs

```rust
use colon_db::ColonDB;
```
</p>


#### Methods:
find save file, or create one
```rust
let mut database = ColonDB::find_database("db.txt");
```
Make sure to ``.to_string()`` input values.
Check example.

add item or row
```rust
database.insert_item_into_db("01".to_string(),"age".to_string(), "18".to_string());
database.insert_item_into_db("01".to_string(),"salary".to_string(), "0".to_string());

database.insert_row_into_db("02".to_string(),vec!["alan".to_string(),"12".to_string(),"23".to_string()]);
```

select a range from the db (0 to total number of rows, vector with column names)
```rust
let newdb = database.select_data(Some(0..4), vec!["name".to_string(),"age".to_string()].into());
// None on either selects all available range
```

get item by key, column
```rust
println!("{}",database.select_item("01", "age").unwrap())
```


delete item, row, column
```rust
database.delete_column("salary".to_string());
database.delete_item("08", "name".to_string());
database.delete_row("09");
```

display database in terminal
```rust
newdb.print_database();
```
