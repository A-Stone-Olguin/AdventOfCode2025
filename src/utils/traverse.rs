use crate::utils::point::Point;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Traverse<'a> {
    current_direction: Direction,
    current_point: Option<Point<'a>>,
}

impl<'a> Traverse<'a> {
    pub fn new(direction: Direction, init_point: Point<'a>) -> Self {
        Traverse {
            current_direction: direction,
            current_point: Some(init_point),
        }
    }

    pub fn step_forward(&mut self) {
        let current_point = match self.current_point {
            Some(p) => p,
            None => return,
        };

        self.current_point = match self.current_direction {
            Direction::Up => current_point.up(),
            Direction::Down => current_point.down(),
            Direction::Left => current_point.left(),
            Direction::Right => current_point.right(),
        };
    }

    pub fn turn_right(&mut self) {
        self.current_direction = match self.current_direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }

    pub fn turn_around(&mut self) {
        self.turn_right();
        self.turn_right();
    }

    pub fn turn_left(&mut self) {
        self.turn_around();
        self.turn_right();
    }
}

#[cfg(test)]
pub mod traverse_test {
    use crate::utils::parse_input_data::parse_input_to_grid;
    use crate::utils::point::Point;
    use crate::utils::traverse::{Direction, Traverse};

    const EXAMPLE_INPUT: &str = r#"
    1234567890
    1234567890
    1234567890
    1234567890
    "#;

    #[test]
    fn test_step_forward() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let initial_point = Point::new(&grid, 0, 0);
        let initial_direction = Direction::Right;

        let mut traverse = Traverse::new(initial_direction, initial_point);

        let expected_point = Point::new(&grid, 1, 0);
        traverse.step_forward();
        let new_point = traverse.current_point.unwrap_or(initial_point);
        assert_eq!(expected_point, new_point);
    }

    #[test]
    fn test_turn_right_then_step() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let start = Point::new(&grid, 1, 1);
        let mut traverse = Traverse::new(Direction::Up, start);

        traverse.turn_right();
        traverse.step_forward();

        assert_eq!(traverse.current_point, Some(Point::new(&grid, 2, 1)));
    }

    #[test]
    fn test_turn_left_then_step() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let start = Point::new(&grid, 2, 1);
        let mut traverse = Traverse::new(Direction::Up, start);

        traverse.turn_left();
        traverse.step_forward();

        assert_eq!(traverse.current_point, Some(Point::new(&grid, 1, 1)));
    }

    #[test]
    fn test_step_forward_off_grid_stops_traversal() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let start = Point::new(&grid, 0, 0);
        let mut traverse = Traverse::new(Direction::Left, start);

        traverse.step_forward();
        assert_eq!(traverse.current_point, None);

        traverse.step_forward();
        assert_eq!(traverse.current_point, None);
    }
}
