
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

```sh
cargo add colon_db
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
database.append_row_to_db("23".to_string(),vec!["a".to_string(),"12".to_string(),"3000".to_string()]);

database.insert_item_into_db("21".to_string(),"name".to_string(), "kak".to_string());
database.insert_item_into_db("21".to_string(),"age".to_string(), "18".to_string());
```

append a column
```rust
db.append_column("status".to_string(), "citizen".to_string());
```

select a range from the db (0 to total number of rows, vector with column names)
```rust
let newdb = database.select_data(Some(4..17), vec!["name".to_string(), "salary".to_string()].into());
```

get item by key, column
```rust
println!("{}",database.select_item("01", "age").unwrap())
```


delete a row
```rust
database.delete_row("09");
```

display database in terminal
```rust
newdb.print_database();
```
