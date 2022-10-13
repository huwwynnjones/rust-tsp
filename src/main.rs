mod citykey;
mod permutation;

use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::citykey::{cities_from_city_keys, CityKey};
use crate::permutation::Permutations;

fn main() {
    let costs = load_costs_from_file();
    let cities = cities_from_city_keys(&costs);
    let permutations = Permutations::new(&cities);
    let mut cheapest_journeys = Vec::new();
    let mut lowest_cost = i32::MAX;

    for journey in permutations {
        let city_pairs = journey_to_city_pairs(&journey);
        let current_cost = calculate_cost(&city_pairs, &costs);
        match current_cost.cmp(&lowest_cost) {
            Ordering::Less => {
                cheapest_journeys.clear();
                cheapest_journeys.push(journey);
                lowest_cost = current_cost
            }
            Ordering::Equal => cheapest_journeys.push(journey),
            _ => (),
        }
    }
    println!(
        "Lowest cost {}, journeys {:?}",
        lowest_cost, cheapest_journeys
    );
}

fn load_costs_from_file() -> HashMap<CityKey, i32> {
    let mut costs = HashMap::new();
    let city_data = match File::open("cities.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };
    let buf_reader = BufReader::new(city_data);

    for line in buf_reader.lines().flatten() {
        let map_input = string_to_map_entry(&line);
        costs.insert(map_input.0, map_input.1);
    }
    costs
}

fn journey_to_city_pairs<'a>(journey: &[&'a str]) -> Vec<[&'a str; 2]> {
    let mut cities = VecDeque::new();
    cities.extend(journey);
    let mut result = Vec::new();

    while cities.len() > 1 {
        let first = cities.pop_front().unwrap();
        let second = cities.front().unwrap();
        result.push([first, *second]);
    }
    result
}

fn string_to_map_entry(input: &str) -> (CityKey, i32) {
    let mut split = input.split_whitespace();
    let start = split.next().expect("No start city");
    let end = split.next().expect("No end city");
    let cost = split
        .next()
        .expect("No cost data")
        .parse::<i32>()
        .expect("Not a number");
    let city_key = CityKey::new(start, end);
    (city_key, cost)
}

fn calculate_cost(city_pairs: &[[&str; 2]], costs: &HashMap<CityKey, i32>) -> i32 {
    city_pairs
        .iter()
        .map(|p| {
            let mut key = CityKey::from(p);
            costs.get(&key).unwrap_or_else(|| {
                costs
                    .get(&key.reverse_key())
                    .unwrap_or_else(|| panic!("No cost for {:?}", p))
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        let mut costs = HashMap::new();
        costs.insert(CityKey::new("A", "B"), 30);
        costs.insert(CityKey::new("B", "C"), 50);
        let city_pairs = vec![["A", "B"], ["B", "C"]];
        assert_eq!(calculate_cost(&city_pairs, &costs), 80)
    }

    #[test]
    fn test_string_to_map_entry() {
        let city_key = CityKey::new("A", "B");
        let input = "A B 80";
        assert_eq!(string_to_map_entry(input), (city_key, 80))
    }

    #[test]
    fn test_journey_to_city_pairs_strings() {
        let journey = vec!["A", "B", "C", "D", "E"];
        let correct_result = vec![["A", "B"], ["B", "C"], ["C", "D"], ["D", "E"]];
        assert_eq!(journey_to_city_pairs(&journey), correct_result)
    }
}
