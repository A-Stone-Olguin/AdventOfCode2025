use ndarray::Array2;

#[derive(PartialEq, Eq, Debug)]
pub struct Point<'a> {
  grid: &'a Array2<char>,
  x: usize,
  y: usize,
}

// impl<'a> Default for Point<'a> {
//     fn default() -> Self {
//         Self { grid: Default::default(), x: Default::default(), y: Default::default() }
//     }
// }

impl<'a> Point<'a> {
  pub fn new(grid: &'a Array2<char>, initial_x: usize, initial_y: usize) -> Self  {
    Self { grid, x: initial_x, y: initial_y }
  }

  pub fn left(&self) -> Option<Point<'_>> {
    match self.x == 0 {
        true => None,
        false => Some(Point{grid: self.grid, x: self.x-1, y: self.y}),
    }
  }

  pub fn right(&self) -> Option<Point<'_>> {
    match self.x >= self.grid.dim().0 - 1 {
        true => None,
        false => Some(Point{grid: self.grid, x: self.x+1, y: self.y}),
    }
  }

  pub fn down(&self) -> Option<Point<'_>> {
    match self.y >= self.grid.dim().1 - 1 {
        true => None,
        false => Some(Point{grid: self.grid, x: self.x, y: self.y+1}),
    }
  }

  pub fn up(&self) -> Option<Point<'_>> {
    match self.y == 0 {
        true => None,
        false => Some(Point{grid: self.grid, x: self.x, y: self.y-1}),
    }
  }

  pub fn value(&self) -> Option<&char> {
    self.grid.get((self.x, self.y))
  }

  pub fn neighbors(&self) -> Array2<Option<&char>> {
    let directive_grid = vec![
      vec![
        vec![Point::up, Point::left],
        vec![Point::up],
        vec![Point::up, Point::right],
      ],
      vec![vec![Point::left], vec![], vec![Point::right]],
      vec![
        vec![Point::down, Point::left],
        vec![Point::down],
        vec![Point::down, Point::right],
      ],
    ];

    let rows = directive_grid.len();
    let cols = directive_grid.first().map(|row| row.len()).unwrap_or(0);
    let mut neighbors = Array2::from_elem((rows, cols), None);

    // TODO: THIS IS TRANSPOSED SOMEHOW
    for (row_idx, row) in directive_grid.iter().enumerate() {
      for (col_idx, chain) in row.iter().enumerate() {
        let final_value = chain.iter().fold(
          Some((self.x, self.y)),
          |acc, directive| {
            acc.and_then(|(x, y)| {
              let point = Point {
                grid: self.grid,
                x,
                y,
              };
              directive(&point).map(|next| (next.x, next.y))
            })
          },
        ).and_then(|(x, y)| self.grid.get((x, y)));

        neighbors[(row_idx, col_idx)] = final_value;
      }
    }

    neighbors
  }

}

#[cfg(test)]
pub mod point_test {
    use crate::utils::point::Point;
    use crate::utils::parse_input_data::parse_input_to_grid;

    const EXAMPLE_INPUT: &str = r#"
    1234567890
    1234567890
    1234567890
    1234567890
    "#;


    #[test]
    fn test_value() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let point = Point::new(&grid, 0, 0);
        assert_eq!(point.value(), Some(&'1'));

        let point = Point::new(&grid, 1, 2);
        assert_eq!(point.value(), Some(&'3'));

        let invalid_point = Point::new(&grid, 100, 0);
        assert_eq!(invalid_point.value(), None);
    }

    #[test]
    fn test_up() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let point = Point::new(&grid, 0, 1);
        let top_point = Point::new(&grid, 0, 0);

        assert_eq!(point.up(), Some(Point::new(&grid, 0, 0)));
        assert_eq!(top_point.up(), None);
    }

    #[test]
    fn test_left() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let point = Point::new(&grid, 1, 0);
        let leftmost_point = Point::new(&grid, 0, 0);

        assert_eq!(point.left(), Some(Point::new(&grid, 0, 0)));
        assert_eq!(leftmost_point.left(), None);
    }

    #[test]
    fn test_right() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let point = Point::new(&grid, 2, 0);
        let rightmost_point = Point::new(&grid, grid.dim().0 - 1, 0);

        assert_eq!(point.right(), Some(Point::new(&grid, 3, 0)));
        assert_eq!(rightmost_point.right(), None);
    }

    #[test]
    fn test_down() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let point = Point::new(&grid, 0, 1);
        let bottom_point = Point::new(&grid, 0, grid.dim().1 - 1);

        assert_eq!(point.down(), Some(Point::new(&grid, 0, 2)));
        assert_eq!(bottom_point.down(), None);
    }

    #[test]
    fn test_neighbors() {
        let grid = parse_input_to_grid(EXAMPLE_INPUT);
        let point = Point::new(&grid, 1, 1);
        let neighbors = point.neighbors();
        println!("{:?}", neighbors);

        let expectations = [
            ((0, 0), (0, 0)),
            ((0, 1), (1, 0)),
            ((0, 2), (2, 0)),
            ((1, 0), (0, 1)),
            ((1, 1), (1, 1)),
            ((1, 2), (2, 1)),
            ((2, 0), (0, 2)),
            ((2, 1), (1, 2)),
            ((2, 2), (2, 2)),
        ];

        for (matrix_coord, grid_coord) in expectations {
            assert_eq!(neighbors[matrix_coord], grid.get(grid_coord));
        }

        let corner_point = Point::new(&grid, 0, 0);
        let corner_neighbors = corner_point.neighbors();
        assert_eq!(corner_neighbors[(0, 0)], None);
        assert_eq!(corner_neighbors[(0, 1)], None);
        assert_eq!(corner_neighbors[(1, 0)], None);
    }
}
