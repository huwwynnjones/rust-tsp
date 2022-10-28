use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
};

use internment::Intern;

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

pub fn cities_from_city_keys(costs: &HashMap<CityKey, i32>) -> Vec<Intern<String>> {
    let city_keys = costs.keys().collect::<Vec<&CityKey>>();
    let mut cities = city_keys
        .iter()
        .flat_map(|k| [k.start(), k.end()])
        .collect::<Vec<Intern<String>>>();
    cities.sort();
    cities.dedup();
    cities
}

pub fn journey_to_city_pairs(journey: &[Intern<String>]) -> Vec<(Intern<String>, Intern<String>)> {
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

pub fn string_to_map_entry(input: &str) -> (CityKey, i32) {
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

pub fn calculate_cost(
    city_pairs: &[(Intern<String>, Intern<String>)],
    costs: &HashMap<CityKey, i32>,
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

pub fn factorial(number: i64) -> i64 {
    (1..=number).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cities_from_city_keys() {
        let mut costs = HashMap::new();
        costs.insert(
            CityKey::new(Intern::from_ref("A"), Intern::from_ref("B")),
            30,
        );
        costs.insert(
            CityKey::new(Intern::from_ref("B"), Intern::from_ref("C")),
            50,
        );
        let correct_result = vec![
            Intern::from_ref("A"),
            Intern::from_ref("B"),
            Intern::from_ref("C"),
        ];
        assert_eq!(cities_from_city_keys(&costs), correct_result)
    }

    #[test]
    fn test_calculate_cost() {
        let mut costs = HashMap::new();
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

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(0), 1);
    }
}
