#![deny(clippy::all)]
fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Parse entire input into a list
    let masses: Vec<usize> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    // Part 1
    let fuel: usize = masses.iter().map(|mass| mass_to_fuel(*mass)).sum();
    println!("Part 1: {}", fuel);

    // Part 2
    let fuel_with_fuel: usize = masses
        .iter()
        .map(|mass| mass_to_fuel_with_fuel(*mass))
        .sum();
    println!("Part 2: {}", fuel_with_fuel);
}

/// Calculate the required fuel for a given mass, not taking the mass of the
/// added fuel into account. Formula: take a mass, divide by three, round down,
/// and subtract 2. Negative fuel is not considered, returns zero.
fn mass_to_fuel(mass: usize) -> usize {
    let fuel = mass / 3;
    fuel.saturating_sub(2)
}

/// Calculate the required fuel for a given mass, taking the mass of the
/// added fuel into account. Formula: calculate fuel required for the mass,
/// recurse by calculating the fuel required for that additional fuel, until
/// reaching zero.
fn mass_to_fuel_with_fuel(mass: usize) -> usize {
    let mut fuel_added = mass_to_fuel(mass);
    let mut total_fuel = fuel_added;
    loop {
        let fuel_for_fuel = mass_to_fuel(fuel_added);
        if fuel_for_fuel == 0 {
            break;
        }
        fuel_added = fuel_for_fuel;
        total_fuel += fuel_added;
    }
    total_fuel
}
