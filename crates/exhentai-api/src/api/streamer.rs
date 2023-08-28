use serde::Deserialize;

#[derive(Deserialize)]
pub struct HitomiImages {
    pub(crate) hashs: Vec<String>,
}
