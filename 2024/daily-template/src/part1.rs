#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("{{crate_name}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
