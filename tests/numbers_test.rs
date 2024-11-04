use pest::Parser;
use anyhow::anyhow;
use ua_contact_book::*;

#[cfg(test)]
mod numbers_test{
    use super::*;
    #[test]
    fn numbers_creation_test()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::numbers, "0904643439, +380904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        let long_pair = Grammar::parse(Rule::numbers, "+380904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "0904643439, +380904643439");
        assert_eq!(long_pair.as_str(), "+380904643439");
        Ok(())
    }

    #[test]
    fn wrong_separator()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::numbers, "+380904643439. 0904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "+380904643439");
        Ok(())
    }

    #[test]
    fn no_value_after_coma()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::numbers, "+380904643439,")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "+380904643439");
        Ok(())
    }

    #[test]
    fn no_value_with_coma()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::numbers, ",");
        assert!(pair.is_err());
        Ok(())
    }

    #[test]
    fn wrong_number()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::numbers, "380904643439");
        assert!(pair.is_err());
        Ok(())
    }
}