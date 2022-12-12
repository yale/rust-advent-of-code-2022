use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coords(usize, usize);

#[derive(Clone, Debug)]
struct Point {
    letter: char,
    height: u8
}

impl Point {
    fn new(letter: char) -> Self {
        let height: u8 = letter.try_into().unwrap();

        Self {
            letter,
            height: if letter.is_lowercase() {
                height - 96
            } else if letter == 'S' {
                1
            } else {
                26
            }
        }
    }

    fn is_start(&self) -> bool {
        self.letter == 'S'
    }

    fn is_end(&self) -> bool {
        self.letter == 'E'
    }
}

struct Grid {
    points: Vec<Vec<Point>>,
    start: Coords,
    end: Coords
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut start = Coords(0, 0);
        let mut end = Coords(0, 0);
        let points = input.split("\n").enumerate().map(|(y, letters)| {
            letters
                .chars()
                .enumerate()
                .map(|(x, letter)| {
                    let coords = Coords(x, y);
                    let point = Point::new(letter);
                    if point.is_start() {
                        start = coords;
                    } else if point.is_end() {
                        end = coords;
                    }
                    point
                })
            .collect()
        }).collect();

        Self {
            points,
            start,
            end
        }
    }

    fn get(&self, coords: Coords) -> Option<&Point> {
        self.points.get(coords.1)?.get(coords.0)
    }

    fn neighbor_candidates(&self, coords: Coords) -> Vec<Coords> {
        let mut neighbors: Vec<Coords> = vec![];

        if coords.0 > 0 { neighbors.push(Coords(coords.0 - 1, coords.1)); }
        if coords.0 < self.points[0].len() - 1 { neighbors.push(Coords(coords.0 + 1, coords.1)); }
        if coords.1 > 0 { neighbors.push(Coords(coords.0, coords.1 - 1)); }
        if coords.1 < self.points.len() - 1 { neighbors.push(Coords(coords.0, coords.1 + 1)); }

        neighbors
    }

    fn neighbors(&self, coords: Coords, a_is_free: bool) -> Vec<(Coords, usize)> {
        let point = self.get(coords).unwrap();
        let candidates = self.neighbor_candidates(coords);
        candidates.iter().filter(|coord| {
            let other = self.get(**coord).unwrap();
            other.height <= point.height + 1
        }).map(|c| {
            let cost = if a_is_free && point.letter == 'a' { 0 } else { 1 };
            (*c, cost)
        }).collect()
    }
}

pub fn shortest_dist(input: &str, a_is_free: bool) -> usize {
    let grid = Grid::parse(input);
    let result = dijkstra(&grid.start, |p| grid.neighbors(*p, a_is_free), |p| grid.end == *p).unwrap();

    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        let result = Grid::parse(input);
        assert_eq!(result.points.len(), 5);
        assert_eq!(result.start, Coords(0,0));
        assert_eq!(result.end, Coords(5,2));

        let start = result.get(Coords(0, 0)).unwrap();
        assert_eq!(start.letter, 'S');

        let end = result.get(Coords(5, 2)).unwrap();
        assert_eq!(end.letter, 'E');

        let neighbors = result.neighbors(Coords(3, 4), false);
        assert_eq!(neighbors, vec![(Coords(2, 4), 1), (Coords(4, 4), 1)]);

        assert_eq!(shortest_dist(input, false), 31);
        assert_eq!(shortest_dist(input, true), 29);
    }
}
