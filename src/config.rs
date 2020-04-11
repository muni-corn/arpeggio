use crate::palette::Palette;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {}

pub type Palettes = HashMap<String, Palette>;
