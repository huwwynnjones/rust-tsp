mod permutation;

use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use crate::permutation::Permutations;

fn main() {
    let mut costs = HashMap::new();

    let cost_string = "A B 20\nA C 30\nB C 40\n";
    for line in cost_string.lines() {
        let map_input = string_to_map_entry(line);
        costs.insert(map_input.0, map_input.1);
    }

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
    println!("Lowest cost {}, journeys {:?}", lowest_cost, cheapest_journeys);
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct CityKey {
    start: String,
    end: String,
}

impl CityKey {
    fn new(start: &str, end: &str) -> Self {
        CityKey {
            start: start.into(),
            end: end.into(),
        }
    }

    fn from(city_pair: &[&str]) -> Self {
        CityKey {
            start: city_pair[0].into(),
            end: city_pair[1].into(),
        }
    }

    fn reverse_key(&mut self) -> Self {
        CityKey {
            start: self.end.clone(),
            end: self.start.clone(),
        }
    }
}

fn cities_from_city_keys<'a>(costs: &'a HashMap<CityKey, i32>) -> Vec<&'a str> {
    let city_keys = costs.keys().collect::<Vec<&CityKey>>();
    let mut cities = city_keys.iter().map(|k| [k.start.as_str(), k.end.as_str()]).flatten().collect::<Vec<&str>>();
    cities.sort();
    cities.dedup();
    cities
}

fn journey_to_city_pairs<'a>(journey: &[&'a str]) -> Vec<[&'a str; 2]> {
    let mut vd = VecDeque::new();
    vd.extend(journey);
    let mut result = Vec::new();

    while vd.len() > 1 {
        let first = vd.pop_front().unwrap();
        let second = vd.front().unwrap();
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
    use std::collections::VecDeque;

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
    fn test_cities_from_city_keys() {
        let mut costs = HashMap::new();
        costs.insert(CityKey::new("A", "B"), 30);
        costs.insert(CityKey::new("B", "C"), 50);
        let correct_result = vec!["A", "B", "C"];
        assert_eq!(cities_from_city_keys(&costs), correct_result)
    }

    #[test]
    fn test_string_to_map_entry() {
        let city_key = CityKey::new("A", "B");
        let input = "A B 80";
        assert_eq!(string_to_map_entry(input), (city_key, 80))
    }

    #[test]
    fn test_vecdeque() {
        let mut vd = VecDeque::from([1, 2, 3, 4, 5]);
        let correct_result = vec![[1, 2], [2, 3], [3, 4], [4, 5]];
        let mut result = Vec::new();

        while vd.len() > 1 {
            let first = vd.pop_front().unwrap();
            let second = vd.front().unwrap().clone();
            result.push([first, second]);
        }
        assert_eq!(result, correct_result)
    }

    #[test]
    fn test_journey_to_city_pairs_strings() {
        let journey = vec!["A", "B", "C", "D", "E"];
        let correct_result = vec![["A", "B"], ["B", "C"], ["C", "D"], ["D", "E"]];
        assert_eq!(journey_to_city_pairs(&journey), correct_result)
    }
}
