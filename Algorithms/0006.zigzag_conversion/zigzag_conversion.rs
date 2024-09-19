impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows];
        let mut row_index = 0;
        let mut direction = 1;

        for c in s.chars() {
            rows[row_index].push(c);
            row_index = (row_index as i32 + direction) as usize;

            if row_index == 0 || row_index == num_rows - 1 {
                direction *= -1;
            }
        }

        rows.concat()
    }
}