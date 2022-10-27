use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
};

use crate::citykey::CityKey;

pub fn load_costs_from_file(filename: &str) -> Result<HashMap<CityKey, i32>, Error> {
    let mut costs = HashMap::new();
    let city_data = File::open(filename)?;
    let buf_reader = BufReader::new(city_data);
    for line in buf_reader.lines().flatten() {
        let map_input = string_to_map_entry(&line);
        costs.insert(map_input.0, map_input.1);
    }
    Ok(costs)
}

pub fn cities_from_city_keys(costs: &HashMap<CityKey, i32>) -> Vec<&str> {
    let city_keys = costs.keys().collect::<Vec<&CityKey>>();
    let mut cities = city_keys
        .iter()
        .map(|k| [k.start(), k.end()])
        .flatten()
        .collect::<Vec<&str>>();
    cities.sort();
    cities.dedup();
    cities
}

pub fn journey_to_city_pairs<'a>(journey: &[&'a str]) -> Vec<[&'a str; 2]> {
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

pub fn string_to_map_entry(input: &str) -> (CityKey, i32) {
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

pub fn calculate_cost(city_pairs: &[[&str; 2]], costs: &HashMap<CityKey, i32>) -> i32 {
    city_pairs
        .iter()
        .map(|pair| {
            let mut key = CityKey::from(pair);
            costs.get(&key).unwrap_or_else(|| {
                costs
                    .get(&key.reverse_key())
                    .unwrap_or_else(|| panic!("No cost for {:?}", pair))
            })
        })
        .sum()
}

pub fn factorial(number: i64) -> i64 {
    (1..=number).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cities_from_city_keys() {
        let mut costs = HashMap::new();
        costs.insert(CityKey::new("A", "B"), 30);
        costs.insert(CityKey::new("B", "C"), 50);
        let correct_result = vec!["A", "B", "C"];
        assert_eq!(cities_from_city_keys(&costs), correct_result)
    }

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

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(0), 1);
    }
}
