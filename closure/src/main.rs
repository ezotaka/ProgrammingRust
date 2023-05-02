fn main() {
    println!("Hello, world!");
    // create 5 cities Vec
    let mut cities = Vec::new();
    cities.push(City {
        population: 100_000,
        monster_attack_risk: 0.5,
    });
    cities.push(City {
        population: 1_000_000,
        monster_attack_risk: 0.0,
    });
    cities.push(City {
        population: 500_000,
        monster_attack_risk: 0.2,
    });
    cities.push(City {
        population: 250_000,
        monster_attack_risk: 0.0,
    });
    cities.push(City {
        population: 750_000,
        monster_attack_risk: 0.1,
    });

    let my_key_fn: fn(&City) -> i64 = city_population_descending;

    cities.sort_by_key(my_key_fn);

    println!("{:?}", cities);

    let count = count_selected_cities(&cities, has_monster_attack_risk);
    println!("{} cities have a risk of monster attack", count);

    let n = count_selected_cities(&cities, |city| city.monster_attack_risk > 0.15);
    println!("{} cities have a risk of monster attack", n);
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

#[derive(Debug)]
struct City {
    population: i64,
    monster_attack_risk: f64,
}

fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn has_monster_attack_risk(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}
