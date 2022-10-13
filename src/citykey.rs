use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct CityKey {
    start: String,
    end: String,
}

impl CityKey {
    pub fn new(start: &str, end: &str) -> Self {
        CityKey {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn from(city_pair: &[&str]) -> Self {
        CityKey {
            start: city_pair[0].into(),
            end: city_pair[1].into(),
        }
    }

    pub fn reverse_key(&mut self) -> Self {
        CityKey {
            start: self.end.clone(),
            end: self.start.clone(),
        }
    }
}

pub fn cities_from_city_keys<'a>(costs: &'a HashMap<CityKey, i32>) -> Vec<&'a str> {
    let city_keys = costs.keys().collect::<Vec<&CityKey>>();
    let mut cities = city_keys
        .iter()
        .map(|k| [k.start.as_str(), k.end.as_str()])
        .flatten()
        .collect::<Vec<&str>>();
    cities.sort();
    cities.dedup();
    cities
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
}
