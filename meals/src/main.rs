use std::str::FromStr;

use clap::Parser;
use meals_lib::{Amount, Day, Ingredient, Meal, MealDay, Unit};

#[derive(clap::Parser)]
struct Args {
    #[arg(long, value_parser = clap::value_parser!(MealDay))]
    days: Vec<MealDay> // should take a path
}

fn main() {
}

