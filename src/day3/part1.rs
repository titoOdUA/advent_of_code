use std::time::SystemTime;

pub fn day3_part1() {
    println!("--------------DAY 3 part1--------------");
    let start_time = SystemTime::now();
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let bit_sum_by_possition = get_bits_sum_by_possition(&input);
    let gamma_rate = calculate_gamma_rate(&bit_sum_by_possition, &input.len());
    let epsilon_rate = calculate_epsilon_rate(&bit_sum_by_possition, &input.len());

    let result = gamma_rate * epsilon_rate;

    println!("result = {}", result);
    println!(
        "calculated in: {} secs",
        start_time.elapsed().unwrap().as_secs_f32()
    );
}

pub fn get_bits_sum_by_possition(input: &Vec<&str>) -> Vec<u32> {
    let mut bits_info = vec![0; input.first().unwrap().len()];
    for line in input {
        for (i, char) in line.chars().enumerate() {
            if let Some(e) = char.to_digit(10) {
                bits_info[i] += e
            }
        }
    }
    bits_info
}

fn calculate_gamma_rate(bits_sum_by_possition: &Vec<u32>, input_len: &usize) -> usize {
    let bits_sequence = get_bit_string(bits_sum_by_possition, true, input_len);
    usize::from_str_radix(&bits_sequence, 2).unwrap()
}

fn get_bit_string(
    bits_sum_by_possition: &Vec<u32>,
    by_most_common: bool,
    input_len: &usize,
) -> String {
    let mut bits_sequence = String::new();
    for bit in bits_sum_by_possition {
        let middle: u32 = (input_len / 2).try_into().unwrap();
        let char = if by_most_common {
            if *bit >= middle {
                '1'
            } else {
                '0'
            }
        } else if *bit <= middle {
            '1'
        } else {
            '0'
        };
        bits_sequence.push(char);
    }
    bits_sequence
}

fn calculate_epsilon_rate(bits_sum_by_possition: &Vec<u32>, input_len: &usize) -> usize {
    let bits_sequence = get_bit_string(bits_sum_by_possition, false, input_len);

    usize::from_str_radix(&bits_sequence, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day3::part1::{
        calculate_epsilon_rate, calculate_gamma_rate, get_bits_sum_by_possition,
    };

    #[test]
    fn calculate_power_consumption_test() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let bit_sum_by_possition = get_bits_sum_by_possition(&input);
        let gamma_rate = calculate_gamma_rate(&bit_sum_by_possition, &input.len());
        let epsilon_rate = calculate_epsilon_rate(&bit_sum_by_possition, &input.len());

        let result = gamma_rate * epsilon_rate;

        assert_eq!(result, 198);
    }

    #[test]
    fn calculate_gamma_rate_test() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let bit_sum_by_possition = get_bits_sum_by_possition(&input);
        let result = calculate_gamma_rate(&bit_sum_by_possition, &input.len());

        assert_eq!(result, 22);
    }

    #[test]
    fn calculate_epsilon_rate_test() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let bit_sum_by_possition = get_bits_sum_by_possition(&input);
        let result = calculate_epsilon_rate(&bit_sum_by_possition, &input.len());

        assert_eq!(result, 9);
    }
}
