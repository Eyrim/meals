use std::{fmt::Display, path::PathBuf, str::FromStr};

pub mod files;
pub mod handling;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct MealDay {
    pub day: Day,
    pub meals: Vec<Meal>,
}

impl FromStr for MealDay {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_json = files::read_to_str(PathBuf::from(s));

        serde_json::from_str(&raw_json)
    }
}

impl Display for MealDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Day {
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Meal {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amount: Amount,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Amount {
    pub unit: Unit,
    pub value: f32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum Unit {
    Count,
    Gram,
    Kilogram,
    Millilitre,
    Litre,
}
