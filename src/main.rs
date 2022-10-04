mod permutation;

use std::vec;

use crate::permutation::Permutations;

fn main() {
    let costs = vec![vec![0, 20, 30], vec![20, 0, 40], vec![30, 40, 0]];
    let a = (1..4).collect::<Vec<_>>();
    let p = Permutations::new(&a);
    let mut journeys = Vec::new();
    let mut cost = i32::MAX;
    for j in p {
        let current_cost = calculate_cost(&j, &costs);
        if current_cost < cost {
            journeys.clear();
            journeys.push(j);
            cost = current_cost;
        } else if current_cost == cost {
            journeys.push(j)
        }
    }
    println!("Lowest cost {}, journeys {:?}", cost, journeys);
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
    use super::*;

    #[test]
    fn test_calculate_cost() {
        let journey = vec![1, 2, 3];
        let costs = vec![vec![0, 20, 30], vec![20, 0, 40], vec![30, 40, 0]];
        assert_eq!(calculate_cost(&journey, &costs), 20 + 40);
        let journey = vec![2, 1, 3];
        assert_eq!(calculate_cost(&journey, &costs), 20 + 30);
    }
}
