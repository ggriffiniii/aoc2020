use crate::split_once;
use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}
impl<'a> Food<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let (ingredients, allergens) = split_once(input, " (contains ")?;
        let allergens = allergens.strip_suffix(")")?;
        Some(Food {
            ingredients: ingredients.split(" ").collect(),
            allergens: allergens.split(", ").collect(),
        })
    }
}

#[aoc(day21, part1)]
fn solve_d2_p1(input: &str) -> usize {
    //let input = EXAMPLE;
    let foods: Vec<_> = input.split('\n').map(|l| Food::parse(l).unwrap()).collect();

    let mut allergen_causes = HashMap::new();
    for food in &foods {
        for &allergen in &food.allergens {
            let ingredients = allergen_causes
                .entry(allergen)
                .or_insert_with(|| food.ingredients.clone());
            ingredients.retain(|ingredient| food.ingredients.contains(ingredient));
        }
    }
    let possible_allergens: HashSet<_> = allergen_causes.values().flatten().collect();
    foods
        .iter()
        .flat_map(|food| &food.ingredients)
        .filter(|ingredient| !possible_allergens.contains(ingredient))
        .count()
}

#[aoc(day21, part2)]
fn solve_d2_p2(input: &str) -> String {
    let foods: Vec<_> = input.split('\n').map(|l| Food::parse(l).unwrap()).collect();

    let mut allergen_causes = HashMap::new();
    for food in &foods {
        for &allergen in &food.allergens {
            let ingredients = allergen_causes
                .entry(allergen)
                .or_insert_with(|| food.ingredients.clone());
            ingredients.retain(|ingredient| food.ingredients.contains(ingredient));
        }
    }
    let mut dangerous_ingredients = Vec::new();
    while let Some(&allergen) = allergen_causes
        .iter()
        .find(|(_k, v)| v.len() == 1)
        .map(|(k, _v)| k)
    {
        let (allergen, mut ingredient) = allergen_causes.remove_entry(allergen).unwrap();
        let ingredient = ingredient.drain().next().unwrap();
        dangerous_ingredients.push((allergen, ingredient));
        for ingredient_set in allergen_causes.values_mut() {
            ingredient_set.remove(&ingredient);
        }
    }
    dangerous_ingredients.sort_by(|(a, _), (b, _)| a.cmp(b));
    dangerous_ingredients
        .into_iter()
        .map(|(_allergen, ingredient)| ingredient)
        .collect::<Vec<_>>()
        .join(",")
}
