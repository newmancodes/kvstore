use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);

    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    match database.flush() {
        Ok(()) => println!("Saved!"),
        Err(err) => println!("Oh No! Error! {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    has_been_flushed: bool,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("Corrupt database - no key!");
            let value = chunks.next().expect("Corrupt database - no value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map, has_been_flushed: false })
    }

    fn insert(&mut self, key: String, value: String) -> () {
        self.map.insert(key, value);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.has_been_flushed = true;
        Database::do_flush(&self)
    }

    fn do_flush(database: &Database) -> std::io::Result<()> {
        println!("Flushing!");
        let mut contents = String::new();

        for (key, value) in &database.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write("kv.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) -> () {
        if !self.has_been_flushed {
            let _ = Database::do_flush(self);
        }
    }
}
