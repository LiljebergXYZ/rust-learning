use std::fs;

pub fn compute() -> i32 {
    let input = fs::read_to_string("input/input1.txt")
        .expect("Something went wrong reading the input");

    let mut total_fuel = 0;
    for num in input.split_whitespace().rev() {
        total_fuel += calculate_fuel(num.parse::<i32>().unwrap());
    }

    return total_fuel;
}

pub fn compute2() -> i32 {
    let input = fs::read_to_string("input/input1.txt")
        .expect("Something went wrong reading the input");

    let mut total_fuel = 0;
    for num in input.split_whitespace().rev() {
        let mass = num.parse::<i32>().unwrap();
        let fuel = calculate_fuel(mass);
        total_fuel += fuel;
        let mut fuel_fuel = calculate_fuel(fuel);
        while fuel_fuel > 0 {
            total_fuel += fuel_fuel;
            fuel_fuel = calculate_fuel(fuel_fuel);
        }
    }

    return total_fuel;
}

fn calculate_fuel(num: i32) -> i32 {
    return num / 3 - 2;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compute() {
        assert_eq!(super::compute(), 3427972);
    }

    #[test]
    fn test_compute2() {
        assert_eq!(super::compute2(), 5139078);
    }
}
