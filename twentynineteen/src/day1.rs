use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<f32>().unwrap())
        .map(|mass| (mass / 3.0).floor() as i32 - 2)
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(total_fuel_from_mass)
        .sum()
}

fn total_fuel_from_mass(mass: i32) -> i32 {
    let fuel = ((mass as f32 / 3.0).floor() - 2.0) as i32;
    let mut total_fuel = fuel;
    // dbg!(total_fuel);
    if fuel > 0 {
        let temp_fuel = total_fuel_from_mass(fuel);
        if temp_fuel > 0 {
            total_fuel += temp_fuel
        }
        return total_fuel;
    }
    fuel
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn sample1() {
        assert_eq!(966, total_fuel_from_mass(1969));
    }
    #[test]
    fn sample2() {
        assert_eq!(50346, total_fuel_from_mass(100756));
    }
    #[test]
    fn sample3() {
        assert_eq!(2, total_fuel_from_mass(14))
    }
}