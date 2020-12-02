#[derive(Debug)]
struct PasswordRule {
    letter: char,
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct PasswordRecord {
    rule: PasswordRule,
    password: String,
}

fn main() {
    let mut password_records: Vec<PasswordRecord> = vec![];
    const INPUT: &str = include_str!("input.txt");
    for line in INPUT.lines() {
        let mut parts = line.split(" ");
        let mut minmax = parts.next().unwrap().split("-");
        let min: usize = minmax.next().unwrap().parse().unwrap();
        let max: usize = minmax.next().unwrap().parse().unwrap();
        // parsing as string, but assuming we only get and care about first letter to make it char
        let letter = parts
            .next()
            .unwrap()
            .replace(":", "")
            .chars()
            .nth(0)
            .unwrap();
        let rule = PasswordRule { letter, min, max };

        let password = parts.next().unwrap().to_owned();
        let password_record = PasswordRecord { rule, password };
        password_records.push(password_record);
    }

    // Part 1
    let mut valid_passwords = 0;
    for record in &password_records {
        let mut count = 0;
        for c in record.password.chars() {
            if c == record.rule.letter {
                count += 1;
            }
        }

        if count >= record.rule.min && count <= record.rule.max {
            valid_passwords += 1;
        }
    }
    println!("Part 1: {}", valid_passwords);

    // Part 2
    let mut valid_passwords = 0;
    for record in &password_records {
        let match1 = record.password.chars().nth(record.rule.min - 1).unwrap();
        let match1 = match1 == record.rule.letter;
        let match2 = record.password.chars().nth(record.rule.max - 1).unwrap();
        let match2 = match2 == record.rule.letter;
        // logical xor
        if (match1 || match2) && !(match1 && match2) {
            valid_passwords += 1;
        }
    }
    println!("Part 2: {}", valid_passwords);
}
