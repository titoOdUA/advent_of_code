use std::time::SystemTime;

pub fn day3_part2() {
    println!("--------------DAY 3 part2--------------");
    let start_time = SystemTime::now();
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();

    println!("result = not implemented yet");
    println!(
        "calculated in: {} secs",
        start_time.elapsed().unwrap().as_secs_f32()
    );
}

fn calculate_oxygen_generator_rating(input: &Vec<&str>) -> usize {
    todo!()
}

fn calculate_co2_rating(input: &[&str]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day3::part2::{calculate_co2_rating, calculate_oxygen_generator_rating};

    #[test]
    fn calculate_oxygen_generator_rating_test() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let result = calculate_oxygen_generator_rating(&input);

        assert_eq!(result, 23);
    }

    #[test]
    fn calculate_co2_rating_test() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let result = calculate_co2_rating(&input);

        assert_eq!(result, 10);
    }

    #[test]
    fn calculate_life_support_test() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let result = calculate_co2_rating(&input) * calculate_oxygen_generator_rating(&input);

        assert_eq!(result, 230);
    }
}