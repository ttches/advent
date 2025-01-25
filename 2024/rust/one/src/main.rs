mod data;

fn get_ordered_sum(data: &str) -> i32 {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();

    // split string into two columns
    data.split("\n").for_each(|line| {
        let pair: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        left_vec.push(pair[0]);
        right_vec.push(pair[1]);
    });

    left_vec.sort();
    right_vec.sort();

    // get sum of absolute difference between the two sorted columns
    let sum: i32 = left_vec
        .iter()
        .enumerate()
        .map(|(index, value)| (value - right_vec[index]).abs())
        .sum();

    sum
}

fn main() {
    let sum = get_ordered_sum(&data::ADVENT_DATA);

    println!("{:#?}", sum);
}
