use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Extensions {
    pub recommendations: Vec<String>,
}
