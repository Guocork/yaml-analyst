use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: String
}


fn main() {
    let file_path = std::path::Path::new("user.yml");

    let yaml_str = std::fs::read_to_string(file_path).unwrap();

    let user:User = serde_yaml::from_str(yaml_str.as_str()).unwrap();

    println!("{:#?}", user)
}

