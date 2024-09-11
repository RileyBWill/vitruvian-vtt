use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VitruvianTheme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub background_alt: String,
    pub font_color: String,
}