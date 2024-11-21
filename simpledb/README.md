
# simpleDB | key value database file store

Checkout colondb folder for multiple column support.
Make sure you clone this repo inside the directory you write in the cargo.toml's ``[dependencies]  simple_db = ...``
May still contain issues, havent tested enough.

---
### usage

<p>
add to Cargo.toml

```toml
[package]
name = "libtest"
version = "0.1.0"
edition = "2021"

[dependencies]
simple_db = { path = "../../simpledb" }
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
let mut database = SimpleDB::find_database("db.txt");
```
You have to ``.to_string()`` input values.
Check example.

add (key, value) pair
```rust
database.insert_into_db(key, value);
```
get value by id (key)
```rust
database.get_value_from_db(key)
```

delete value by key
```rust
database.delete_from_db(key)
```


