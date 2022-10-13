mod citykey;
mod permutation;

use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use ahash::AHashMap;
use internment::Intern;

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

fn load_costs_from_file() -> AHashMap<CityKey, i32> {
    let mut costs = AHashMap::new();
    let city_data = File::open("cities.txt").expect("Failed to open file");
    let buf_reader = BufReader::new(city_data);
    for line in buf_reader.lines().flatten() {
        let map_input = string_to_map_entry(&line);
        costs.insert(map_input.0, map_input.1);
    }
    costs
}

fn journey_to_city_pairs(journey: &[Intern<String>]) -> Vec<(Intern<String>, Intern<String>)> {
    let mut cities = VecDeque::new();
    cities.extend(journey);
    let mut result = Vec::new();

    while cities.len() > 1 {
        let first = cities.pop_front().expect("Missing city in city pairs");
        let second = cities.front().expect("Missing city in city pairs");
        result.push((first, *second));
    }
    result
}

fn string_to_map_entry(input: &str) -> (CityKey, i32) {
    let mut split = input.split_whitespace();
    let s = split.next().expect("No start city");
    let start = Intern::new(s.to_string());
    let e = split.next().expect("No end city");
    let end = Intern::new(e.to_string());
    let cost = split
        .next()
        .expect("No cost data")
        .parse::<i32>()
        .expect("Not a number");
    let city_key = CityKey::new(start, end);
    (city_key, cost)
}

fn calculate_cost(
    city_pairs: &[(Intern<String>, Intern<String>)],
    costs: &AHashMap<CityKey, i32>,
) -> i32 {
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
        let mut costs = AHashMap::new();
        costs.insert(
            CityKey::new(Intern::from_ref("A"), Intern::from_ref("B")),
            30,
        );
        costs.insert(
            CityKey::new(Intern::from_ref("B"), Intern::from_ref("C")),
            50,
        );
        let city_pairs = vec![
            (Intern::from_ref("A"), Intern::from_ref("B")),
            (Intern::from_ref("B"), Intern::from_ref("C")),
        ];
        assert_eq!(calculate_cost(&city_pairs, &costs), 80)
    }

    #[test]
    fn test_string_to_map_entry() {
        let city_key = CityKey::new(Intern::from_ref("A"), Intern::from_ref("B"));
        let input = "A B 80";
        assert_eq!(string_to_map_entry(input), (city_key, 80))
    }

    #[test]
    fn test_journey_to_city_pairs_strings() {
        let journey = vec![
            Intern::from_ref("A"),
            Intern::from_ref("B"),
            Intern::from_ref("C"),
            Intern::from_ref("D"),
            Intern::from_ref("E"),
        ];
        let correct_result = vec![
            (Intern::from_ref("A"), Intern::from_ref("B")),
            (Intern::from_ref("B"), Intern::from_ref("C")),
            (Intern::from_ref("C"), Intern::from_ref("D")),
            (Intern::from_ref("D"), Intern::from_ref("E")),
        ];
        assert_eq!(journey_to_city_pairs(&journey), correct_result)
    }
}
