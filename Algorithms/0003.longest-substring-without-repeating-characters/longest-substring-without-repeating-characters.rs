use std::cmp::max;

pub fn longest_substring_without_repeating_characters (s: String) -> usize {

    let mut start = 0;
    let mut end = 0;

    let s_length = s.len();
    let mut sub_strings: Vec<char> = vec![];

    let mut result = 0;

    while end < s_length {
        let c = s.chars().nth(end).unwrap();

        let mut loop_finish = true;
        let mut i = 0;
        for vv in sub_strings.iter() {
            if vv == &c {
                start = i + start + 1;
                end = start - 1;
                loop_finish = false;
                sub_strings = vec![];
                break;
            }
            i += 1;
        }

        if loop_finish {
            sub_strings.push(c);
        }

        end += 1;
        result = max(result, end - start);
    }

    return result;
}

fn main() {
    let s = String::from("Hello world");
    let result = longest_substring_without_repeating_characters(s);

    println!("{}", result);
}
