mod citykey;
mod permutations;
mod util;

use std::cmp::Ordering;

use crate::{
    permutations::Permutations,
    util::{
        calculate_cost, cities_from_city_keys, factorial, journey_to_city_pairs,
        load_costs_from_file,
    },
};

fn main() {
    let costs = match load_costs_from_file("cities.txt") {
        Ok(c) => c,
        Err(err) => panic!("Unable to load file {}", err),
    };
    let cities = cities_from_city_keys(&costs);
    let permutations = Permutations::new(&cities);
    let mut cheapest_journeys = Vec::new();
    let mut lowest_cost = i32::MAX;

    println!("Number of permutations {}", factorial(cities.len() as i64));

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
