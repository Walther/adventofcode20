use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // (row, column), ID
    let mut seats: HashMap<(usize, usize), usize> = HashMap::new();

    for line in INPUT.lines() {
        // Split the row and column components
        let row_instructions = line[0..7].to_owned();
        let column_instructions = line[7..].to_owned();

        let mut row_estimator: Vec<usize> = (0..128).collect();
        for instruction in row_instructions.chars() {
            let len = row_estimator.len();
            let half = len / 2;
            match instruction {
                'F' => row_estimator = row_estimator[0..half].to_owned(),
                'B' => row_estimator = row_estimator[half..].to_owned(),
                _ => panic!("Invalid instruction: {}", instruction),
            }
        }

        assert!(row_estimator.len() == 1);
        let row = row_estimator[0];

        let mut column_estimator: Vec<usize> = (0..8).collect();
        for instruction in column_instructions.chars() {
            let len = column_estimator.len();
            let half = len / 2;
            match instruction {
                'L' => column_estimator = column_estimator[0..half].to_owned(),
                'R' => column_estimator = column_estimator[half..].to_owned(),
                _ => panic!("Invalid instruction: {}", instruction),
            }
        }

        assert!(column_estimator.len() == 1);
        let column = column_estimator[0];

        seats.insert((row, column), row * 8 + column);
    }

    // Part 1
    let highest_id = seats.values().max().unwrap();
    println!("Part 1: {}", highest_id);

    // Part 2
    let mut first_seat: Option<(usize, usize)> = None;
    let mut my_seat: Option<(usize, usize)> = None;
    let mut last_seat: Option<(usize, usize)> = None;
    for row in 0..128 {
        // print!("{:0>3}: ", row); // visualization
        for column in 0..8 {
            if let Some(_id) = seats.get(&(row, column)) {
                // print!("#"); // visualization
                if first_seat.is_none() {
                    first_seat = Some((row, column));
                }
            } else {
                // print!(".") // visualization
                if first_seat.is_some() && my_seat.is_none() {
                    my_seat = Some((row, column));
                } else if my_seat.is_some() && last_seat.is_none() {
                    last_seat = Some((row, column - 1));
                }
            }
        }
        // println!(); // visualization
    }
    let my_seat = my_seat.unwrap();

    println!("Part 2: {}", my_seat.0 * 8 + my_seat.1);
}
