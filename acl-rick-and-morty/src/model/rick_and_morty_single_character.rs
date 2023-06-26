use serde::{ Deserialize, Serialize };

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: i32, 
    pub name: String, 
    pub status: String, 
    #[serde(alias = "type")]
    pub my_type: String, 
    pub gender: String,
    pub origin: Origin,
    pub location: Location,
    pub image: String, 
    pub episode: Vec<String>,
    pub url: String,
    pub created: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Origin {
    pub name: String, 
    pub url: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String, 
    pub url: String
}