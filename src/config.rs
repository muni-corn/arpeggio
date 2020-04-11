use std::collections::HashMap;
use crate::palette::Palette;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config { }

pub type Palettes = HashMap<String, Palette>;
