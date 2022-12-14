use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let part_one_ans: usize = calc("./src/day_1/part1.txt", 2);
    let part_two_ans: usize = calc("./src/day_1/part2.txt", 4);
    (part_one_ans as u64, part_two_ans as u64)
}

fn calc(path: &str, window: usize) -> usize {
    let measurements = read_one_per_line::<u32>(&path).unwrap();
    let windowed: Vec<&[u32]> = measurements
        .windows(window)
        .filter(|items| items[0] < items[items.len() - 1])
        .collect();
    windowed.len()
}
