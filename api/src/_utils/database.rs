use regex::Regex;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct DBRecord {
  pub id: Thing,
}

#[derive(Debug, Deserialize)]
pub struct DBCount {
  pub count: u32,
}

pub fn db_thing_to_id(thing: &Thing) -> Option<u32> {
  let raw_thing = thing.to_raw(); // table_name:{ id : 123}
  let re = Regex::new(r":\s(\d+)");
  match re {
    Ok(re) => {
      if let Some(captures) = re.captures(&raw_thing) {
        let id = captures.get(1).unwrap().as_str().parse::<u32>();
        match id {
          Ok(id) => Some(id),
          Err(_) => None,
        }
      } else {
        None
      }
    }
    Err(_) => None,
  }
}
