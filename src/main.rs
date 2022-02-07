use std::collections::HashMap;

fn main() {
    // code to get first two arguments in console
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}

struct Todo {
    // implement hashmap
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {

        // create content variable as String
        let mut content = String::new();

        // loop through hashmap
        // format as String with newlines
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }

        // save content variable as txt file on disk
        std::fs::write("db.txt", content)
    }
}
