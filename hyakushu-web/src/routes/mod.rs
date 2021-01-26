use serde::{Deserialize, Serialize};

pub mod question_sets;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum JSNumberType {
    Float(f64),
    Str(Option<String>),
}
