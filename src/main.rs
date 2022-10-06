mod permutation;

use std::collections::{HashMap, VecDeque};

use crate::permutation::Permutations;

fn main() {
    // let costs = vec![vec![0, 20, 30], vec![20, 0, 40], vec![30, 40, 0]];
    // let a = (1..4).collect::<Vec<_>>();
    // let p = Permutations::new(&a);
    // let mut journeys = Vec::new();
    // let mut cost = i32::MAX;
    // for j in p {
    //     let current_cost = calculate_cost(&j, &costs);
    //     if current_cost < cost {
    //         journeys.clear();
    //         journeys.push(j);
    //         cost = current_cost;
    //     } else if current_cost == cost {
    //         journeys.push(j)
    //     }
    // }
    // println!("Lowest cost {}, journeys {:?}", cost, journeys);

    let mut cost_map = HashMap::new();

    let cost_string = "A B 80\nB C 30\n";
    for line in cost_string.lines() {
        let map_input = string_to_map_entry(line);
        cost_map.insert(map_input.0, map_input.1);
    }

    println!("{:?}", cost_map)

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
        CityKey { start: city_pair[0].into(), end: city_pair[1].into()}
    }

    fn reverse_key(&mut self) -> Self {
        CityKey {
            start: self.end.clone(),
            end: self.start.clone(),
        }
    }
}

fn journey_to_city_pairs<T>(journey: &[T]) -> Vec<[&T;2]> {

        let mut vd = VecDeque::new();
        vd.extend(journey);
        let mut result = Vec::new();

        while vd.len() > 1 {
            let first = vd.pop_front().unwrap();
            let second = vd.front().unwrap();
            result.push([first, second]);
        }
        result
}


fn string_to_map_entry(input: &str) -> (CityKey, i32) {
    let mut split = input.split_whitespace();
    let start = split.next().expect("No data in input string");
    let end = split.next().expect("No data in input string");
    let cost = split
        .next()
        .expect("No cost data")
        .parse::<i32>()
        .expect("Not a number");
    let city_key = CityKey::new(start, end);
    (city_key, cost)
}

fn calculate_cost1(city_pairs: &Vec<[&str; 2]>, costs: HashMap<CityKey, i32>) ->i32 {
    city_pairs.iter().map(|p| costs.get(&CityKey::from(p)).unwrap_or(&0)).sum()
}

fn calculate_cost(journey: &[i32], costs: &Vec<Vec<i32>>) -> i32 {
    let mut cost = 0;
    for (idx, city) in journey.iter().enumerate() {
        let start = (city - 1) as usize;
        if idx < (journey.len() - 1) {
            let end = (journey[idx + 1] - 1) as usize;
            cost += costs[start][end];
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn test_calculate_cost() {
        let journey = vec![1, 2, 3];
        let costs = vec![vec![0, 20, 30], vec![20, 0, 40], vec![30, 40, 0]];
        assert_eq!(calculate_cost(&journey, &costs), 20 + 40);
        let journey = vec![2, 1, 3];
        assert_eq!(calculate_cost(&journey, &costs), 20 + 30);
    }

    #[test]
    fn test_calculate_cost1() {
        let mut costs = HashMap::new();
        costs.insert(CityKey::new("A", "B"), 30);
        costs.insert(CityKey::new("B", "C"), 50);
        let city_pairs = vec!(["A", "B"], ["B", "C"]);
        assert_eq!(calculate_cost1(&city_pairs, costs), 80)
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
    fn test_journey_to_city_pairs_numbers() {
        let journey = vec![1, 2, 3, 4, 5];
        let correct_result = vec![[&1, &2], [&2, &3], [&3, &4], [&4, &5]];
        assert_eq!(journey_to_city_pairs(&journey), correct_result)
    }
    
    #[test]
    fn test_journey_to_city_pairs_strings() {
        let journey = vec!["A", "B", "C", "D", "E"];
        let correct_result = vec![[&"A", &"B"], [&"B", &"C"], [&"C", &"D"], [&"D", &"E"]];
        assert_eq!(journey_to_city_pairs(&journey), correct_result)
    }
}
