use std::collections::HashSet;

pub fn calculate_visible_trees(input: &str) -> usize {
    let data: Vec<Vec<u32>> = input
        .split("\n")
        .map(str::trim)
        .map(str::chars)
        .map(|l| l.map(|c| c.to_digit(10).unwrap_or(0)).collect())
        .collect();

    let num_rows = data.len();
    let num_cols = data[0].len();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for i in 1..(num_rows - 1) {
        let mut max_left = data[i][0];
        let mut max_right = data[i][num_cols - 1];
        let mut max_top = data[0][i];
        let mut max_bottom = data[num_rows - 1][i];

        for j in 1..(num_cols - 1) {
            let from_left = data[i][j];
            let from_right = data[i][num_cols - j - 1];
            let from_top = data[j][i];
            let from_bottom = data[num_rows - j - 1][i];

            if from_left > max_left {
                visible.insert((i, j));
                max_left = from_left;
            }

            if from_right > max_right {
                visible.insert((i, num_cols - j - 1));
                max_right = from_right;
            }

            if from_top > max_top {
                visible.insert((j, i));
                max_top = from_top;
            }

            if from_bottom > max_bottom {
                visible.insert((num_rows - j - 1, i));
                max_bottom = from_bottom;
            }
        }
    };

    visible.len() + 2 * num_rows + 2 * num_cols - 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = "30373\n\
                       25512\n\
                       65332\n\
                       33549\n\
                       35390";

        dbg!(example);
        assert_eq!(calculate_visible_trees(example), 21);
    }
}
