use regex::Regex;
use std::convert::TryFrom;

/// MaybePassport: has the fields of a Passport but may have missing values
#[derive(Debug, Clone)]
struct MaybePassport {
    /// Birth Year
    byr: Option<String>,
    /// Issue Year
    iyr: Option<String>,
    /// Expiration Year
    eyr: Option<String>,
    /// Height
    hgt: Option<String>,
    /// Hair Color
    hcl: Option<String>,
    /// Eye Color
    ecl: Option<String>,
    /// Passport ID
    pid: Option<String>,
    /// Country ID
    cid: Option<String>,
}

impl Default for MaybePassport {
    fn default() -> Self {
        MaybePassport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

/// FullPassport: has all the required fields for a valid passport, but not necessarily valid values
#[derive(Debug)]
struct FullPassport {
    /// Birth Year
    byr: String,
    /// Issue Year
    iyr: String,
    /// Expiration Year
    eyr: String,
    /// Height
    hgt: String,
    /// Hair Color
    hcl: String,
    /// Eye Color
    ecl: String,
    /// Passport ID
    pid: String,
    /// Country ID
    cid: Option<String>,
}

impl TryFrom<MaybePassport> for FullPassport {
    type Error = &'static str;

    fn try_from(value: MaybePassport) -> Result<Self, Self::Error> {
        Ok(FullPassport {
            byr: value.byr.ok_or("Missing byr")?,
            iyr: value.iyr.ok_or("Missing iyr")?,
            eyr: value.eyr.ok_or("Missing eyr")?,
            hgt: value.hgt.ok_or("Missing hgt")?,
            hcl: value.hcl.ok_or("Missing hcl")?,
            ecl: value.ecl.ok_or("Missing ecl")?,
            pid: value.pid.ok_or("Missing pid")?,
            cid: value.cid,
        })
    }
}

impl FullPassport {
    fn is_valid(&self) -> bool {
        true

        // Birth year between 1920 and 2002
        && {
            match self.byr.parse::<usize>() {
                Ok(byr) => byr >= 1920 && byr <= 2002,
                Err(_) => false,
            }
        }
        // Issue year between 2010 and 2020
        && {
            match self.iyr.parse::<usize>() {
                Ok(iyr) => iyr >= 2010 && iyr <= 2020,
                Err(_) => false,
            }
        }

        // Expiration year between 2020 and 2030
        && {
            match self.eyr.parse::<usize>() {
                Ok(eyr) => eyr >= 2020 && eyr <= 2030,
                Err(_) => false,
            }
        }

        // Height: a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        && {
            // TODO: this is super ugly, fix it :x
            let len = self.hgt.len();
            let height = &self.hgt[..len-2];
            let unit = &self.hgt[len-2..len];
            match height.parse::<usize>() {
                Ok(height) => {
                    match unit {
                        "cm" => {
                            height >= 150 && height <= 193
                        },
                        "in" => {
                            height >= 59 && height <= 76
                        },
                        _ => false
                    }
                },
                Err(_) => false
            }
        }

        // Hair color: a # followed by exactly six characters 0-9 or a-f.
        // Ugly regex
        && {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(&self.hcl)
        }

        // Eye color: exactly one of: amb blu brn gry grn hzl oth.
        && {
            match self.ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false
            }
        }
        // Passport id: a nine-digit number, including leading zeroes.
        // Ugly regex
        && {
            let re = Regex::new(r"^[0-9]{9}$").unwrap();
            re.is_match(&self.pid)
        }
        // Country id: ignored, missing or not.
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Different passports are separated by two newlines
    let passport_data: Vec<String> = INPUT.split("\n\n").map(|s| s.to_owned()).collect();
    // Same passport may have multiple lines
    let passport_data: Vec<String> = passport_data.iter().map(|p| p.replace("\n", " ")).collect();
    // Each passport has multiple fields
    let passport_data: Vec<Vec<String>> = passport_data
        .iter()
        .map(|p| p.split(" ").map(|s| s.to_owned()).collect())
        .collect();

    // Rest of the owl: parse fields into MaybePassports, then validate into Passports
    let mut maybe_passports: Vec<MaybePassport> = Vec::new();
    for passport_input in passport_data {
        let mut maybe_passport = MaybePassport::default();
        for field in passport_input {
            let mut key_val = field.split(":");
            let key = key_val.next().unwrap();
            let val = key_val.next().unwrap();
            match key {
                "byr" => maybe_passport.byr = Some(val.to_owned()),
                "iyr" => maybe_passport.iyr = Some(val.to_owned()),
                "eyr" => maybe_passport.eyr = Some(val.to_owned()),
                "hgt" => maybe_passport.hgt = Some(val.to_owned()),
                "hcl" => maybe_passport.hcl = Some(val.to_owned()),
                "ecl" => maybe_passport.ecl = Some(val.to_owned()),
                "pid" => maybe_passport.pid = Some(val.to_owned()),
                "cid" => maybe_passport.cid = Some(val.to_owned()),
                _ => panic!("Invalid passport field"),
            }
        }
        maybe_passports.push(maybe_passport);
    }

    // Part 1
    let full_passports: Vec<FullPassport> = maybe_passports
        .iter()
        .filter_map(|p| FullPassport::try_from(p.clone()).ok())
        .collect();

    println!("Part 1: {}", full_passports.len());

    let valid_passports = full_passports.iter().filter(|p| p.is_valid()).count();
    println!("Part 2: {}", valid_passports);
}
