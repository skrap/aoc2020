use std::{collections::HashMap, mem::take};

use regex::Regex;

fn read_recs(input: &str) -> Vec<HashMap<String,String>> {
    let keyval_re = Regex::new(r"(\S+):(\S+)").unwrap();

    let mut result = vec![];
    let mut pending = HashMap::new();
    for line in input.lines() {
        if line == "" {
            result.push(take(&mut pending));
        } else {
            for keyval in keyval_re.captures_iter(line) {
                let key = &keyval[1];
                let val = &keyval[2];
                pending.insert(key.into(), val.into());
            }
        }
    }
    if !pending.is_empty() {
        result.push(pending);
    }
    result
}

fn is_valid(rec: &HashMap<String,String>) -> bool {
    rec.contains_key("byr") &&
    rec.contains_key("iyr") &&
    rec.contains_key("eyr") &&
    rec.contains_key("hgt") &&
    rec.contains_key("hcl") &&
    rec.contains_key("ecl") &&
    rec.contains_key("pid")
}

fn part1(input: &str) -> usize {
    let recs = read_recs(input);
    recs.iter().filter(|rec| is_valid(*rec)).count()
}

/*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
*/
fn is_extra_valid(rec: &HashMap<String,String>) -> Option<()> {
    let four_digits = Regex::new(r"^\d\d\d\d$").unwrap();

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = rec.get("byr")?;
    if !(four_digits.is_match(byr) && (1920..=2002).contains(&byr.parse::<i32>().unwrap())) {
        return None;
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = rec.get("iyr")?;
    if !(four_digits.is_match(iyr) && (2010..=2020).contains(&iyr.parse().unwrap())) {
        return None;
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = rec.get("eyr")?;
    if !(four_digits.is_match(eyr) && (2020..=2030).contains(&eyr.parse().unwrap())) {
        return None;
    }
    
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    let hgt = rec.get("hgt")?;
    let hgt_re = Regex::new(r"^(\d+)((in|cm))$").unwrap();
    let hgt_caps = hgt_re.captures(hgt)?;
    let hgt_val = hgt_caps[1].parse::<i32>().unwrap();
    if &hgt_caps[2] == "in" && ! (59..=76).contains(&hgt_val) {
        return None;
    }
    if &hgt_caps[2] == "cm" && ! (150..=193).contains(&hgt_val) {
        return None;
    }


    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    if ! Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(rec.get("hcl")?) {
        return None;
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    if ! &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&rec.get("ecl")?) {
        return None;
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    if ! Regex::new(r"^\d{9}$").unwrap().is_match(rec.get("pid")?) {
        return None;
    }

    // cid (Country ID) - ignored, missing or not.
    return Some(())
}

fn part2(input: &str) -> usize {
    let recs = read_recs(input);
    recs.iter().filter(|rec| is_extra_valid(*rec).is_some()).count()
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {} valid passports", part1(input));
    println!("Part 2: {} valid passports", part2(input));
}
