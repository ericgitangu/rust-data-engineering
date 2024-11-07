use serde::{Deserialize, Serialize};
use serde_json;

/// A struct representing a person with a name and age.
///
/// # Examples
///
/// ```
/// use serialize_deserialize_json::Person;
/// use serde_json;
///
/// let person = Person {
///     name: "Alice".to_string(),
///     age: 30,
/// };
///
/// // Serialize to JSON
/// let json = serde_json::to_string(&person).unwrap();
/// assert_eq!(json, r#"{"name":"Alice","age":30}"#);
///
/// // Deserialize back to struct
/// let deserialized_person: Person = serde_json::from_str(&json).unwrap();
/// assert_eq!(deserialized_person.name, "Alice");
/// assert_eq!(deserialized_person.age, 30);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json);

    // Deserialize back to struct
    let deserialized_person: Person = serde_json::from_str(&json).unwrap();
    println!("Deserialized Person: {:?}", deserialized_person);
}
