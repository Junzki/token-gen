// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub uid: String,
//     pub scopes: Vec<String>
// }

#[derive(Deserialize)]
pub struct Spec {
    pub generic: Generic,
    pub payload: Option<Payload>
}

#[derive(Deserialize)]
pub struct Generic {
    pub project: String,
    pub algorithm: String,
    pub secret: String
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub uid: String,
    pub scopes: Vec<String>
}