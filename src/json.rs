use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Json {
    pub version: String,
    pub previous_version: String,
    pub bump: bool,
}

impl ToString for Json {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
