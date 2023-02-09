use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub pages: Option<i64>
}