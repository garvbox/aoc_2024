use glam::UVec2;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let inputs: Vec<Entity> = input
        .lines()
        .rev() // NOTE: Input is processed row by row in reverse so that X-Y coords make sense
        .skip_while(|line| line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| parse_row(row, line))
        .collect();

    let obstructions: Vec<&UVec2> = inputs
        .iter()
        .filter_map(|i| match i {
            Entity::Obstruction(position) => Some(position),
            _ => None,
        })
        .collect();

    let mut guard_position: GuardPosition = inputs
        .iter()
        .filter_map(|i| match i {
            Entity::Guard(guard_position) => Some(guard_position),
            _ => None,
        })
        .next()
        .unwrap()
        .clone();

    tracing::trace!("Obstructions: {:?}", obstructions);
    tracing::trace!("Guard Position: {:?}", guard_position);

    return Ok("".to_string());
}

#[tracing::instrument]
fn parse_row(row: usize, input: &str) -> Vec<Entity> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, ch)| {
            let pos = (row as u32, column as u32);
            match ch {
                '#' => Some(Entity::Obstruction(UVec2::from(pos))),
                '^' => Some(Entity::Guard(GuardPosition::new(pos.0, pos.1))),
                '.' => None,
                _ => unreachable!("Parser missed possible input {:?}", input),
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Entity {
    Obstruction(UVec2),
    Guard(GuardPosition),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct GuardPosition {
    position: UVec2,
    direction: GuardDirection,
}

impl GuardPosition {
    fn new(x: u32, y: u32) -> GuardPosition {
        GuardPosition {
            position: UVec2::from((x, y)),
            direction: GuardDirection::North,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
enum GuardDirection {
    #[default]
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!("41", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple_single_exit() -> miette::Result<()> {
        let input = "
....#.....
..........
....^.....
";
        // Should move up 1 step and then 5 to the right then done
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_empty_row() -> miette::Result<()> {
        let input = "..........";
        let expected: Vec<Entity> = vec![];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(GuardPosition::new(0, 4))];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_non_zero_row() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(GuardPosition::new(7, 4))];
        assert_eq!(expected, parse_row(7, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_and_obstruction() -> miette::Result<()> {
        let input = "....^...#.";
        let expected: Vec<Entity> = vec![
            Entity::Guard(GuardPosition::new(0, 4)),
            Entity::Obstruction(UVec2::new(0, 8)),
        ];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }
}
