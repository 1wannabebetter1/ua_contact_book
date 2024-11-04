use pest::Parser;
use anyhow::anyhow;
use ua_contact_book::*;

#[cfg(test)]
mod name_test{
    use super::*;
    #[test]
    fn name_creation_test()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::name, "Іван Іваненко")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "Іван Іваненко");

        Ok(())
    }
    #[test]
    fn name_with_dot()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::name, "Роберт Дауні мол.")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "Роберт Дауні мол.");
        Ok(())
    }

    #[test]
    fn no_value_name()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::name, "");
        assert!(pair.is_err());
        Ok(())
    }

    #[test]
    fn wrong_format_name()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::name, ",Іван");
        assert!(pair.is_err());
        Ok(())
    }
}