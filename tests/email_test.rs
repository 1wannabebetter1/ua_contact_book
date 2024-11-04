use anyhow::anyhow;
use pest::Parser;
use ua_contact_book::*;

#[cfg(test)]
mod email_test {
    use super::*;
    #[test]
    fn email_creation_test() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::email, "test.user1@examp.le")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "test.user1@examp.le");

        Ok(())
    }

    #[test]
    fn no_value_email() {
        let pair = Grammar::parse(Rule::email, "");
        assert!(pair.is_err());
    }

    #[test]
    fn to_short_domain() {
        let pair = Grammar::parse(Rule::email, "test.user1@e.le");
        assert!(pair.is_err());
    }

    #[test]
    fn nothing_before_at() {
        let pair = Grammar::parse(Rule::email, "@examp.le");
        assert!(pair.is_err());
    }

    #[test]
    fn no_at() {
        let pair = Grammar::parse(Rule::email, "user1.test.edu.ua");
        assert!(pair.is_err());
    }

    #[test]
    fn bad_char_email() {
        let pair = Grammar::parse(Rule::email, "test.use/r1@e.le");
        assert!(pair.is_err());
    }

    #[test]
    fn to_long_user() {
        let pair = Grammar::parse(
            Rule::email,
            "test.usertest.usertest.usertest.usertest.usertest.usertest.usertest.user@ukr.net",
        );
        assert!(pair.is_err());
    }
}
