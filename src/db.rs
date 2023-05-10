use bytes::Bytes;
use std::collections::HashMap;

#[derive(Default)]
pub struct Db {
    entries: HashMap<String, Bytes>,
}

impl Db {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn read(&mut self, arr: &[String]) -> Result<&Bytes, &'static str> {
        let key = &arr[1];
        let query_result = self.entries.get(key);
    
        if let Some(value) = query_result {
            return Ok(value);
        } else {
            return Err("no such key found");
        }
    }
    pub fn write(&mut self, attrs: &[String]) -> Result<&str, &'static str> {
        let key = &attrs[1];
        let value = &attrs[2];

        let val = value.clone();
        let res: &Option<Bytes> = &self.entries.insert(String::from(key), Bytes::from(val));

        match res {
            Some(_res) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }
}
