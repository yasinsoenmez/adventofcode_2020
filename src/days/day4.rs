use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

pub fn get_input() -> String {
    let path = Path::new("input/day4.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn day4() -> (u32, u32) {
    let input = get_input();

    (day4_1(&input), day4_2(&input))
}

pub fn day4_1(input: &str) -> u32 {
    input.split("\n\n")
         .filter(|s| {
             s.contains("byr:") &&
             s.contains("iyr:") &&
             s.contains("eyr:") &&
             s.contains("hgt:") &&
             s.contains("hcl:") &&
             s.contains("ecl:") &&
             s.contains("pid:")
         })
         .count() as u32
}

pub fn day4_2(input: &str) -> u32 {
    input.split("\n\n")
         .filter(|s| {
             let field_num = s.contains("byr:") &&
                             s.contains("iyr:") &&
                             s.contains("eyr:") &&
                             s.contains("hgt:") &&
                             s.contains("hcl:") &&
                             s.contains("ecl:") &&
                             s.contains("pid:");

            if !field_num {
                return false
            }

            let fields = s.lines()
                          .flat_map(|line| {
                              line.split(' ')
                                  .map(|field| {
                                      field.split(':')
                                           .collect_tuple()
                                           .unwrap()

                                  })
                                  .collect::<HashMap<&str, &str>>()
                          })
                          .collect::<HashMap<&str, &str>>();

            let byr = fields.get("byr").unwrap().parse::<i32>().unwrap();
            let iyr = fields.get("iyr").unwrap().parse::<i32>().unwrap();
            let eyr = fields.get("eyr").unwrap().parse::<i32>().unwrap();
            let hcl = fields.get("hcl").unwrap();
            let ecl = fields.get("ecl").unwrap();
            let pid = fields.get("pid").unwrap();

            let hgt = fields.get("hgt").unwrap();
            let hgt_unit = &hgt[hgt.len() - 2..];
            let hgt_i32 = hgt[..hgt.len() - 2].parse::<i32>().unwrap_or_else(|_| 0);
            let hgt = (hgt_unit == "cm" && (150..=193).contains(&hgt_i32)) ||
                      (hgt_unit == "in" && (59..=76).contains(&hgt_i32));
            let ecl_test = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

            field_num &&
                (1920..=2002).contains(&byr) &&
                (2010..=2020).contains(&iyr) &&
                (2020..=2030).contains(&eyr) &&
                pid.len() == 9 &&
                ecl_test.contains(ecl) &&
                hgt &&
                (hcl.len() == 7 && hcl.starts_with("#") && hcl[1..].chars().all(|c| c.is_digit(16)))
         })
         .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test() {
        let (day4_1, day4_2) = day4();
        assert_eq!(day4_1, 213); //213
        assert_eq!(day4_2, 147); //
    }

    #[test]
    fn day4_extra() {
        let input1 = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in");

        let input2 = String::from("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");

        assert_eq!(day4_1(&input1.trim()), 2); //
        assert_eq!(day4_2(&input2.trim()), 4); //
    }
}
