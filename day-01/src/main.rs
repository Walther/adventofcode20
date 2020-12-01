fn main() {
    let mut expenses = Vec::new();

    const INPUT: &str = include_str!("input.txt");
    for line in INPUT.lines() {
        let parsed = line.parse::<i32>().unwrap();
        expenses.push(parsed);
    }

    // Part 1: sum of two numbers == 2020
    'outer1: for (i, expense_a) in expenses.iter().enumerate() {
        for expense_b in expenses[i + 1..].iter() {
            if expense_a + expense_b == 2020 {
                let multiplied = expense_a * expense_b;
                println!("Part 1: {}", multiplied);
                break 'outer1;
            }
        }
    }

    // Part 2: sum of three numbers == 2020
    'outer2: for (i, expense_a) in expenses.iter().enumerate() {
        for (j, expense_b) in expenses[i + 1..].iter().enumerate() {
            for expense_c in expenses[j + 1..].iter() {
                if expense_a + expense_b + expense_c == 2020 {
                    let multiplied = expense_a * expense_b * expense_c;
                    println!("Part 2: {}", multiplied);
                    break 'outer2;
                }
            }
        }
    }
}
