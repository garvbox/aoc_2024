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

    let mut guard: GuardLocation = GuardLocation::new(0, 0);

    let obstructions: Vec<&UVec2> = inputs
        .iter()
        .filter_map(|i| match i {
            Entity::Obstruction(position) => Some(position),
            Entity::Guard(position) => {
                guard.position = position.clone();
                None
            }
        })
        .collect();

    tracing::trace!("Obstructions: {:?}", obstructions);
    tracing::trace!("Guard Position: {:?}", guard);

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
                '^' => Some(Entity::Guard(UVec2::from(pos))),
                '.' => None,
                _ => unreachable!("Parser missed possible input {:?}", input),
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Entity {
    Obstruction(UVec2),
    Guard(UVec2),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct GuardLocation {
    position: UVec2,
    direction: GuardDirection,
}

impl GuardLocation {
    fn new(x: u32, y: u32) -> GuardLocation {
        GuardLocation {
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
        let expected: Vec<Entity> = vec![Entity::Guard(UVec2::new(0, 4))];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_non_zero_row() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(UVec2::new(7, 4))];
        assert_eq!(expected, parse_row(7, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_and_obstruction() -> miette::Result<()> {
        let input = "....^...#.";
        let expected: Vec<Entity> = vec![
            Entity::Guard(UVec2::new(0, 4)),
            Entity::Obstruction(UVec2::new(0, 8)),
        ];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }
}