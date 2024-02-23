pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

impl Direction {
  /// Get the inverse of a given direction.
  ///
  /// Example:
  /// ```
  /// Direction::Left.inverse() // Direction::Right
  /// ```
  pub fn inverse(&self) -> Direction {
    match self {
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
    }
  }

  /// Parse a string into a direction.
  ///
  /// Example:
  /// ```
  /// Direction::from_str("left") // Direction::Left
  /// ```
  pub fn from_str(unparsed: &str) -> Result<Direction> {
    match self {
      "left" => Direction::Left,
      "right" => Direction::Right,
      "up" => Direction::Up,
      "down" => Direction::Down,
      _ => bail!("Not a valid direction: {}", unparsed),
    }
  }
}
