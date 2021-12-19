use std::{env, process, cmp::Ordering};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Usage: maraang [string] [anagram ...]");
        process::exit(1);
    }

    let from_str = &args[0];
    let mut any_fail = false;

    for to_str in &args[1..] {
        if from_str == to_str {
            print_valid(from_str, to_str);
            continue;
        }

        match get_char_diff(from_str, to_str) {
            None => {
                print_valid(from_str, to_str);
            }
            Some((chars_missing, chars_leftover)) => {
                any_fail = true;
                print_invalid(from_str, to_str, chars_missing, chars_leftover);
            }
        }
    }

    if any_fail {
        process::exit(1);
    }
}

fn print_valid(from: &String, to: &String) {
    println!("+ \"{}\" is an anagram of \"{}\".", to, from);
}

fn print_invalid(from: &String, to: &String, chars_missing: Vec<char>, chars_leftover: Vec<char>) {
    println!("- \"{}\" is no anagram of \"{}\".", to, from);

    if chars_missing.len() > 0 {
        println!("\t- Missing characters: {:?}", chars_missing);
    }

    if chars_leftover.len() > 0 {
        println!("\t- Leftover characters: {:?}", chars_leftover);
    }
}

fn get_chars_sorted(s: &String) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    chars.sort_by(|a, b| a.cmp(b));
    chars
}

fn get_char_diff(from_str: &String, to_str: &String) -> Option<(Vec<char>, Vec<char>)> {
    let from_chars = get_chars_sorted(from_str);
    let to_chars = get_chars_sorted(to_str);

    if from_chars == to_chars {
        return None;
    }

    let mut chars_missing: Vec<char> = vec![];
    let mut chars_leftover: Vec<char> = vec![];

    let mut from_i: usize = 0;
    let mut to_i: usize = 0;

    while from_i < from_chars.len() && to_i < to_chars.len() {
        let from_char = from_chars[from_i];
        let to_char = to_chars[to_i];

        if from_char == to_char {
            from_i += 1;
            to_i += 1;
        } else if from_char.cmp(&to_char) == Ordering::Less {
            chars_missing.push(from_char);
            from_i += 1;
        } else {
            chars_leftover.push(to_char);
            to_i += 1;
        }
    }

    chars_missing.extend_from_slice(&from_chars[from_i..]);
    chars_leftover.extend_from_slice(&to_chars[to_i..]);

    Some((
        chars_missing,
        chars_leftover
    ))
}

#[test]
fn test_get_chars_sorted() {
    let test_cases: Vec<(&str, Vec<char>)> = vec![
        ("abc", vec!['a', 'b', 'c']),
        ("cba", vec!['a', 'b', 'c']),
        ("cba fed hjig", ('a'..='j').collect()),
    ];

    for test_case in test_cases {
        assert_eq!(get_chars_sorted(&String::from(test_case.0)), test_case.1);
    }
}

#[test]
fn test_get_char_diff() {
    let test_cases: Vec<(&str, &str, Option<(Vec<char>, Vec<char>)>)> = vec![
        ("anagram", "maraang", None),
        ("bzarfooba", "foobarbaz", None),
        ("b z a r f OOBA", "Foo bar bAz", None),
        ("abc", "ab", Some((vec!['c'], vec![]))),
        ("ab", "abc", Some((vec![], vec!['c']))),
        ("bbb aaa ccc ddd", "bbbb aa cccc dd", Some((vec!['a', 'd'], vec!['b', 'c']))),
    ];

    for test_case in test_cases {
        let result = get_char_diff(&String::from(test_case.0), &String::from(test_case.1));
        assert_eq!(result, test_case.2);
    }
}
