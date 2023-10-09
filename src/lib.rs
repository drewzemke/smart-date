pub fn parse(text: &str) -> Option<()> {
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_crash() {
        let _result = parse("");
    }
}
