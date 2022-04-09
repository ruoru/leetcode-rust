pub struct Solution {}

fn transform(s: &str) -> i32 {
    match s {
        "I" => 1,
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        "IV" => 4,
        "IX" => 9,
        "XL" => 40,
        "XC" => 90,
        "CD" => 400,
        "CM" => 900,
        _ => 0,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s_length = s.len();
        let mut i = 0;
        let mut result = 0;

        while i < s_length {
            let ostr = s.chars().nth(i).unwrap().to_string();
            let mut nstr = String::from("");

            i += 1;
            if i < s_length {
                let nchar = s.chars().nth(i).unwrap();
                nstr.push_str(&ostr);
                nstr.push(nchar);

                let val = transform(&nstr);
                if val != 0 {
                    result += val;
                    i += 1;
                    continue;
                }
            };

            result += transform(&ostr);
        }

        return result;
    }
}
