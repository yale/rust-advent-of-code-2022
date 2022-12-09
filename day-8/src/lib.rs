use itertools::Itertools;
use std::{collections::HashSet, fmt::Debug};

type Trees = Vec<Vec<u32>>;

fn parse_trees(input: &str) -> Trees {
    input
        .split("\n")
        .map(str::trim)
        .map(str::chars)
        .map(|l| l.map(|c| c.to_digit(10).unwrap_or(0)).collect())
        .collect()
}

pub fn calculate_visible_trees(input: &str) -> usize {
    let data = parse_trees(input);
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
    }

    visible.len() + 2 * num_rows + 2 * num_cols - 4
}

pub fn find_dist_to_equal_or_greater_height<I>(trees: &Trees, height: u32, iter: I) -> usize
where
I: ExactSizeIterator<Item = (usize, usize)> + Debug
{
    let total = iter.len();
    match iter
        .enumerate()
        .find(|(n, pt)| {
            trees[pt.0][pt.1] >= height
        })
    {
        Some((n, _pt)) => n + 1,
        None => total,
    }
}

pub fn calculate_scenic_score(trees: &Trees, point: (usize, usize)) -> usize {
    // Maybe pass these in?
    let num_rows = trees.len();
    let num_cols = trees[0].len();
    let height = trees[point.0][point.1];

    let to_top = (0..point.0).rev().map(|x| (x, point.1));
    let to_bottom = ((point.0+1)..num_rows).map(|x| (x, point.1));
    let to_left = (0..point.1).rev().map(|y| (point.0, y));
    let to_right = ((point.1 + 1)..num_cols).map(|y| (point.0, y));

    let score_left = find_dist_to_equal_or_greater_height(trees, height, to_left);
    let score_right = find_dist_to_equal_or_greater_height(trees, height, to_right);
    let score_top = find_dist_to_equal_or_greater_height(trees, height, to_top);
    let score_bottom = find_dist_to_equal_or_greater_height(trees, height, to_bottom);

    score_left * score_right * score_top * score_bottom
}

pub fn calculate_max_scenic_score(input: &str) -> usize {
    let data = parse_trees(input);

    let num_rows = data.len();
    let num_cols = data[0].len();

    let max_scenic_score: usize = (0..num_cols)
        .cartesian_product(0..num_rows)
        .map(|point| calculate_scenic_score(&data, point))
        .max()
        .unwrap();

    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_trees_works() {
        let example = "30373\n\
                       25512\n\
                       65332\n\
                       33549\n\
                       35390";

        assert_eq!(calculate_visible_trees(example), 21);
    }

    #[test]
    fn scenic_score_works() {
        let example = "30373\n\
                       25512\n\
                       65332\n\
                       33549\n\
                       35390";

        assert_eq!(calculate_scenic_score(&parse_trees(example), (3, 2)), 8);
        assert_eq!(calculate_max_scenic_score(example), 8);
    }
}
