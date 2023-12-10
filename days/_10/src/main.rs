use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    join,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!("illegal character in input"),
        }
    }
}

#[derive(Debug)]
struct Map {
    width: u8,
    tiles: Vec<Tile>,
    start: (u8, u8),
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let (width, _) = input.lines().next().unwrap().char_indices().last().unwrap();
        let tiles = input
            .chars()
            .filter_map(|char| {
                if char == '\n' {
                    None
                } else {
                    Some(Tile::from(char))
                }
            })
            .collect::<Vec<_>>();

        let (start, _) = tiles
            .iter()
            .enumerate()
            .find(|(_, &tile)| tile == Tile::Start)
            .unwrap();

        Self {
            width: width as u8 + 1,
            tiles,
            start: ((start % (width + 1)) as u8, (start / (width + 1)) as u8),
        }
    }
}

impl std::ops::Index<(u8, u8)> for Map {
    type Output = Tile;

    fn index(&self, (x, y): (u8, u8)) -> &Self::Output {
        &self.tiles[y as usize * ((self.width) as usize) + x as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Map {
    fn follow(&self, (x, y): (u8, u8), heading: Heading) -> (u32, (u8, u8)) {
        use Heading::*;
        use Tile::*;
        let mut current = (heading, (x, y));
        let mut steps = 0;
        let mut cycle_counter = 0;
        let mut breadcrumbs = Vec::new();
        loop {
            current = match (self[current.1], current.0) {
                (Vertical, North) => (North, (x, y - 1)),
                (Vertical, South) => (South, (x, y + 1)),
                (Vertical, East | West) => return (steps, current.1),

                (Horizontal, West) => (West, (x, y - 1)),
                (Horizontal, East) => (East, (x, y + 1)),
                (Horizontal, North | South) => return (steps, current.1),

                (NorthEast, South) => (East, (x + 1, y)),
                (NorthEast, West) => (North, (x, y - 1)),
                (NorthEast, North | East) => return (steps, current.1),

                (NorthWest, South) => (West, (x - 1, y)),
                (NorthWest, East) => (North, (x, y - 1)),
                (NorthWest, North | West) => return (steps, current.1),

                (SouthWest, North) => (West, (x - 1, y)),
                (SouthWest, East) => (South, (x, y + 1)),
                (SouthWest, South | West) => return (steps, current.1),

                (SouthEast, North) => (East, (x + 1, y)),
                (SouthEast, West) => (South, (x, y + 1)),
                (SouthEast, South | East) => return (steps, current.1),

                (Ground | Start, _) => return (steps, current.1),
            };
            if breadcrumbs.contains(&current) {
                println!("cycle detected");
                return (steps, current.1);
            }
            steps += 1;
            cycle_counter += 1;
            if cycle_counter == 2000 {
                cycle_counter = 0;
                breadcrumbs.push(current);
            }
        }
    }
}

fn a(input: &str) {
    let map = Map::from(input);
    let steps: Vec<_> = [
        ((map.start.0, map.start.1 - 1), Heading::North),
        ((map.start.0 + 1, map.start.1), Heading::East),
        ((map.start.0, map.start.1 + 1), Heading::South),
        ((map.start.0 - 1, map.start.1), Heading::West),
    ]
    .par_iter()
    .map(|&(start, heading)| map.follow(start, heading))
    // .filter(|&(_, end)| map[end] == Tile::Start)
    .map(|(steps, _)| steps)
    .collect();

    println!("{steps:#?}")
}

fn main() {
    let s = r#".....
.S-7.
.|.|.
.L-J.
....."#;
    a(s);
    a(INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_map() {
        let _ = Map::from(INPUT);
    }

    #[test]
    fn index_map() {
        let map = Map::from(INPUT);
        assert_eq!(map[(0, 0)], Tile::Vertical);
        assert_eq!(map[(1, 0)], Tile::Horizontal);
        assert_eq!(map[(map.width - 1, 0)], Tile::NorthWest);
        assert_eq!(map[(0, 1)], Tile::Vertical);
        assert_eq!(map[(0, 2)], Tile::NorthEast);
        assert_eq!(map[(0, 3)], Tile::Vertical);
        assert_eq!(map[(1, 1)], Tile::SouthWest);
    }

    #[test]
    fn index_map2() {
        let map = Map::from(INPUT);
        assert_eq!(map[(map.start.0, map.start.1 - 1)], Tile::NorthWest);
        assert_eq!(map[(map.start.0 + 1, map.start.1)], Tile::NorthEast);
        assert_eq!(map[(map.start.0, map.start.1 + 1)], Tile::NorthEast);
        assert_eq!(map[(map.start.0 - 1, map.start.1)], Tile::SouthEast);
    }

    #[test]
    fn start() {
        let map = Map::from(INPUT);
        assert_eq!(map[map.start], Tile::Start)
    }
}
