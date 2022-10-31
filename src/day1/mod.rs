use std::{fs, time::SystemTime};

pub fn day1() {
    println!("--------------DAY 1--------------");
    let start_time = SystemTime::now();

    let content = fs::read_to_string("src/day1/input.txt").expect("msg");
    let input: Vec<u64> = content
        .split('\n')
        .map(|e| e.parse::<u64>().unwrap())
        .collect();
    let result = compare_measurement_windows(input);

    println!("result = {}", result);
    println!(
        "calculated in: {} secs",
        start_time.elapsed().unwrap().as_secs_f32()
    );
}

fn compare_measurement_windows(input: Vec<u64>) -> usize {
    input
        .windows(3)
        .map(|e| e[0] + e[1] + e[2])
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::compare_measurement_windows;

    #[test]
    fn compare_measurement_windows_test() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = compare_measurement_windows(input);

        assert_eq!(result, 5);
    }
}
