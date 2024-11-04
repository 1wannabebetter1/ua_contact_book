use anyhow::anyhow;
use pest::Parser;
use ua_contact_book::*;

#[cfg(test)]
mod birth_date_test {
    use super::*;
    #[test]
    fn birth_date_creation_test() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::birth_date, "06.03.2004")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "06.03.2004");
        let pair = Grammar::parse(Rule::birth_date, "06/03/2004")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "06/03/2004");
        Ok(())
    }

    #[test]
    fn birth_date_different_delimiter() {
        let pair = Grammar::parse(Rule::birth_date, "06/03.2004");
        assert!(pair.is_err());
    }

    #[test]
    fn no_value_birth_date() {
        let pair = Grammar::parse(Rule::birth_date, "");
        assert!(pair.is_err());
    }

    #[test]
    fn wrong_style_birth_date() {
        let pair = Grammar::parse(Rule::birth_date, "6.3.04");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::birth_date, "06/03/04");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::birth_date, "6.03.2004");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::birth_date, "6/3/2004");
        assert!(pair.is_err());
    }

    #[test]
    fn no_year_birth_date() {
        let pair = Grammar::parse(Rule::birth_date, "06/03");
        assert!(pair.is_err());
    }

    #[test]
    fn wrong_delimiter_birth_date() {
        let pair = Grammar::parse(Rule::birth_date, "06-03-2004");
        assert!(pair.is_err());
    }
}
