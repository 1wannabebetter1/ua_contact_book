use pest::Parser;
use anyhow::anyhow;
use ua_contact_book::*;

#[cfg(test)]
mod number_test{
    use super::*;
    #[test]
    fn number_creation_test()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::number, "0904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        let long_pair = Grammar::parse(Rule::number, "+380904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "0904643439");
        assert_eq!(long_pair.as_str(), "+380904643439");
        Ok(())
    }

    #[test]
    fn number_length_test()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::number, "0904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        let long_pair = Grammar::parse(Rule::number, "+380904643439")?.next().ok_or_else(||anyhow!("No pair"))?;
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 10);
        assert_eq!(long_pair.as_span().start(), 0);
        assert_eq!(long_pair.as_span().end(), 13);
        Ok(())
    }

    #[test]
    fn wrong_format()-> anyhow::Result<()> {
        let pair =  Grammar::parse(Rule::number, "xyz");
        assert!(pair.is_err());
        let pair =  Grammar::parse(Rule::number, "380904643439");
        assert!(pair.is_err());
        Ok(())
    }

}