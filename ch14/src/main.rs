use std::thread;
struct City {
    // name: String,
    population: i64,
    // country: String,
}
impl City {
    fn get_statistic(&self, stat: i64) -> i64 {
        println!("{}", stat);
        stat
    }
}
fn city_population_descending(city: &City) -> i64 {
    -city.population
}
fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending); // ok
}
fn sort_cities_closure1(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

fn start_sorting_thread(mut cities: Vec<City>, stat: i64) -> thread::JoinHandle<Vec<City>> {
    // let key_fn = |city: &City| -> i64 { -city.get_statistic(stat) }; // NG
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    // thread::spawn(move || { // NG
    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}
fn call_twice<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}
fn main() {
    let mut i = 0;
    call_twice(|| i += 1); // ok!
    call_twice(|| println!("{}", i));
    assert_eq!(i, 2);
    // println!("Hello, world!");
}
