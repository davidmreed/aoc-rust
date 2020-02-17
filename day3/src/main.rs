use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug,PartialEq)]
enum Direction { 
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug,PartialEq)]
struct Command {
    dir: Direction,
    dist: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point{x: x, y: y}
    }

    fn walk_command(&self, cmd: &Command) -> Vec<Point> {
        let incr_x = match cmd.dir { Direction::Left => -1, Direction::Right => 1, _ => 0 };
        let incr_y = match cmd.dir { Direction::Up => 1, Direction::Down => -1, _ => 0 };
        let mut points = Vec::new();

        for d in 0..cmd.dist { 
            points.push(Point::new(self.x + incr_x * (d + 1), self.y + incr_y * (d + 1)));
        }
        
        points
    }

    fn walk(&self, path: &Vec<Command>) -> Vec<Point> {
        let mut points = Vec::new();
        let mut pos = self;

        for cmd in path {
            points.append(&mut pos.walk_command(&cmd));
            pos = points.get(points.len() - 1).unwrap();
        }

        points
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn from_string(s: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    for each_move in s.split(',') {
        commands.push(
            Command{
                dir: match &each_move[0..1] {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction")
                },
                dist: each_move[1..].trim().parse().unwrap(),
            }
        );
    }
    commands
}

fn intersections<'a>(wire1: &'a Vec<Point>, wire2: &'a Vec<Point>) -> Vec<&'a Point> {
    let mut wire1_set: HashSet<&Point> = HashSet::new();
    let mut wire2_set: HashSet<&Point> = HashSet::new();

    for pt in wire1 {
        wire1_set.insert(pt);
    }
    for pt in wire2 {
        wire2_set.insert(pt);
    }
    wire1_set.intersection(&wire2_set).map(|pt| *pt).collect()
}

fn steps_to_position(path: &Vec<Point>, pt: &Point) -> i32 {
    (path.iter().position(|r| r == pt).unwrap() + 1).try_into().unwrap()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let wire_commands: Vec<Vec<Command>> = reader.lines().map(|line| from_string(&line.unwrap())).collect();
    let origin = Point::new(0, 0);
    
    // Execute each path walk to get a list of visited points
    let wire_1_path = origin.walk(&wire_commands[0]);
    let wire_2_path = origin.walk(&wire_commands[1]);

    // Find the intersections
    let cross_points = intersections(&wire_1_path, &wire_2_path);
    
    // Find the minimal Manhattan distance
    println!(
        "Minimal Manhattan distance: {}",
        cross_points.iter().map(|inter| origin.manhattan_distance(inter)).min().unwrap()
    );

    // Find the intersection with the shortest total distance traversed.
    println!(
        "Shortest steps: {}",
        cross_points.iter().map(|inter|
            steps_to_position(&wire_1_path, inter) + steps_to_position(&wire_2_path, inter)
        ).min().unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let origin = Point::new(0, 0);

        assert_eq!(3, origin.manhattan_distance(&Point::new(0, 3)));
        assert_eq!(3, origin.manhattan_distance(&Point::new(1, 2)));
        assert_eq!(12, Point::new(-3, -3).manhattan_distance(&Point::new(3, 3)));
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            vec![Command{dir: Direction::Up, dist: 5}, Command{dir: Direction::Right, dist: 10}],
            from_string("U5,R10")
        );
    }

    #[test]
    fn test_walk_command() {
        let origin = Point::new(0, 0);

        assert_eq!(
            origin.walk_command(&Command{dir: Direction::Up, dist: 3}),
            vec![Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)]
        );

        assert_eq!(
            origin.walk_command(&Command{dir: Direction::Down, dist: 3}),
            vec![Point::new(0, -1), Point::new(0, -2), Point::new(0, -3)]
        );

        assert_eq!(
            origin.walk_command(&Command{dir: Direction::Right, dist: 3}),
            vec![Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)]
        );

        assert_eq!(
            origin.walk_command(&Command{dir: Direction::Left, dist: 3}),
            vec![Point::new(-1, 0), Point::new(-2, 0), Point::new(-3, 0)]
        );
    }

    #[test]
    fn test_walk() {
        let path = Point::new(0, 0).walk(
            &vec![
                Command{dir:Direction::Up, dist:5},
                Command{dir:Direction::Right, dist:3},
                Command{dir:Direction::Down, dist:2}
            ]
        );
        println!("{:?}", path);
        assert!(!path.contains(&Point::new(0, 0)));
        assert!(path.contains(&Point::new(0, 1)));
        assert!(path.contains(&Point::new(0, 5)));
        assert!(path.contains(&Point::new(1, 5)));
        assert!(path.contains(&Point::new(3, 5)));
        assert!(path.contains(&Point::new(3, 3)));
    }

    #[test]
    fn test_intersections() {
        let origin = Point::new(0, 0);
        let path_1 = origin.walk(&from_string("R8,U5,L5,D3"));
        let path_2 = origin.walk(&from_string("U7,R6,D4,L4"));
        let cross_points = intersections(&path_1, &path_2);

        println!("{:?}", cross_points);
        assert_eq!(2, cross_points.len());
        assert!(cross_points.contains(&&Point{x:3, y:3}));
        assert!(cross_points.contains(&&Point{x:6, y:5}));
    }

    #[test]
    fn test_steps_to_position() {
        let path = vec![Point::new(0, 1), Point::new(0, 2), Point::new(0, 3), Point::new(0, 4)];

        assert_eq!(2, steps_to_position(&path, &Point::new(0, 2)));
    }
}
