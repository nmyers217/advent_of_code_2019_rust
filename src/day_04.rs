const INPUT_LOW: u32 = 273_025;
const INPUT_HIGH: u32 = 767_253;

pub fn solve() {
    let mut matches = 0;
    let mut strict_matches = 0;
    for password in INPUT_LOW..INPUT_HIGH {
        if password_matches(password, false) {
            matches += 1
        }
        if password_matches(password, true) {
            strict_matches += 1
        }
    }
    println!("{}\n{}", matches, strict_matches);
}

fn password_matches(password: u32, strict_match: bool) -> bool {
    let digits = digits(password);

    let mut left_i = 0;
    let mut right_i = 1;
    let mut is_match = false;
    let mut consecutive = 1;
    while right_i < digits.len() {
        if digits[right_i] < digits[left_i] {
            is_match = false;
            break;
        }

        if digits[left_i] == digits[right_i] {
            consecutive += 1;
        }

        if digits[left_i] != digits[right_i] || right_i + 1 == digits.len() {
            if strict_match && consecutive == 2 || !strict_match && consecutive >= 2 {
                is_match = true;
            }
            consecutive = 1;
        }

        left_i += 1;
        right_i += 1;
    }

    is_match
}

fn digits(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut cur_n = n;
    while cur_n > 0 {
        result.push(cur_n % 10);
        cur_n /= 10;
    }
    result.reverse();
    result
}

#[test]
fn can_validate_a_password() {
    assert_eq!(password_matches(111_111, false), true);
    assert_eq!(password_matches(223_450, false), false);
    assert_eq!(password_matches(123_789, false), false);
    assert_eq!(password_matches(111_222, true), false);
    assert_eq!(password_matches(112_222, true), true);
}
