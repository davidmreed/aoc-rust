use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn fuel_recursive(mass: i32) -> i32 {
    let ret = fuel(mass);

    if ret > 0 {
        ret + fuel_recursive(ret)
    } else {
        0 
    }
}

fn main() {
    let filename = "day1.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        total_fuel += fuel_recursive(line.parse().unwrap());
    }

    println!("Total fuel required: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test_fuel_recursive() {
        assert_eq!(fuel_recursive(14), 2);
        assert_eq!(fuel_recursive(1969), 966);
        assert_eq!(fuel_recursive(100756), 50346);
    }
}
