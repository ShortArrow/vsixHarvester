use serde::Deserialize;

#[derive(Deserialize)]
pub struct Extensions {
    pub recommendations: Vec<String>,
}