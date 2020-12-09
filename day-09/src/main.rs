use std::collections::VecDeque;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let numbers: Vec<usize> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let mut buffer: VecDeque<usize> = VecDeque::with_capacity(25);

    let mut secret_number = 0; // avoids an uninitialized error later on...

    // Part 1
    'part1: for (i, &number) in numbers.iter().enumerate() {
        // Preamble
        if i < 25 {
            buffer.push_back(number);
            continue;
        }

        // Main loop
        let mut found = false;
        'main: for a in buffer.iter() {
            for b in buffer.iter() {
                if a + b == number {
                    found = true;
                    break 'main;
                }
            }
        }

        if !found {
            secret_number = number;
            println!("Part 1: {}", secret_number);
            break 'part1;
        }

        // buffer maintenance
        buffer.push_back(number);
        buffer.pop_front();
    }

    // Part 2
    buffer.clear();
    let mut sum = 0;

    'part2: for (i, &_number) in numbers.iter().enumerate() {
        // inefficient, but should work:
        // start from every number in order
        // grow a buffer until the sum matches or exceeds our target
        //
        // TODO: a more efficient method would be to grow until exceeding the sum,
        // then pop from the other end until below the sum again, then repeat.
        // kind of "worming" across the list
        let mut relative_index = 0;
        'inner: while sum < secret_number {
            if let Some(n) = numbers.get(i + relative_index) {
                buffer.push_back(*n);
                sum = buffer.iter().sum();
                relative_index += 1;
            } else {
                // if we run over the list somehow, continue to the next starting point
                // ... this technically shouldn't happen, the sum should go over the limit faster...
                panic!("ran over the list length");
            }
        }
        if sum == secret_number {
            break 'part2;
        } else {
            sum = 0;
            buffer.clear();
        }
    }
    let min = buffer.iter().min().unwrap();
    let max = buffer.iter().max().unwrap();
    println!("Part 2: {}", min + max);
}
