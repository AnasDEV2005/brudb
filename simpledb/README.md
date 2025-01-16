
# simpleDB | key value database file store

Checkout colondb crate for multiple column support.


> [!NOTE]
> The simple_db struct is not the database itself, 
> its just a way to apply changes to the .txt where the database is saved

---
### usage

<p>
add to Cargo.toml

```sh
cargo add simple_db
```
</p>

<p>
use in main.rs

```rust
use simple_db::SimpleDB;
```
</p>


#### Methods:
find save file, or create one
```rust
let mut database = SimpleDB::find_database("db3.txt");
let mut db = database.unwrap(); // this or handle the error
```
You have to ``.to_string()`` input values.
Like so:
```rust
database.insert_into_db(key, value); // add key value pair to database
```
get value by id (key)
```rust
database.get_value_from_db(key)
```

delete value by key
```rust
database.delete_from_db(key)
```

sort the database
```rust
db.sort_by_key();
db.sort_by_value();
```

to edit the data yourself:
```rust
db.data... // the data here is an indexmap of string: string
```

print db 
```rust
db.print_db()
```
