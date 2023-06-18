use std::collections::HashMap;


pub struct enkovDB<T>{
    data: HashMap<String, T>
}


impl<T: Clone> enkovDB<T> {
    fn new() -> enkovDB<T> {
        enkovDB{
            data: HashMap::new()
        }
    }

    fn insert_one(&self, key: &str, value: T){
        self.data.insert(key.to_string(), value);
    }

    fn insert_many(&self, data: Vec<(&str, T)>){
        for (key, value) in data{
            self.data.insert(key.to_string(), value);
        }
    }


    fn get(&self, key: &str) -> Option<&T>{
        self.data.get(key)
    }
}
