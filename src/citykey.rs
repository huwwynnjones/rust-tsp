use ahash::AHashMap;
use internment::Intern;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct CityKey {
    start: Intern<String>,
    end: Intern<String>,
}

impl CityKey {
    pub fn new(start: Intern<String>, end: Intern<String>) -> Self {
        CityKey { start, end }
    }

    pub fn from(city_pair: &(Intern<String>, Intern<String>)) -> Self {
        CityKey {
            start: city_pair.0,
            end: city_pair.1,
        }
    }

    pub fn reverse_key(&mut self) -> Self {
        CityKey {
            start: self.end,
            end: self.start,
        }
    }
}

pub fn cities_from_city_keys(costs: &AHashMap<CityKey, i32>) -> Vec<Intern<String>> {
    let city_keys = costs.keys().collect::<Vec<&CityKey>>();
    let mut cities = city_keys
        .iter()
        .flat_map(|k| [k.start, k.end])
        .collect::<Vec<Intern<String>>>();
    cities.sort();
    cities.dedup();
    cities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cities_from_city_keys() {
        let mut costs = AHashMap::new();
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
}
