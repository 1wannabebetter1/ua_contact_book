use anyhow::anyhow;
use pest::Parser;
use ua_contact_book::*;

#[cfg(test)]
mod city_test {
    use super::*;
    #[test]
    fn city_creation_test() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::city, "Біла Церква , 09103")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "Біла Церква , 09103");

        Ok(())
    }
    #[test]
    fn city_name_with_dot() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::city, "Вашингтон Д. С., 20001")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "Вашингтон Д. С., 20001");
        Ok(())
    }

    #[test]
    fn no_value_city() {
        let pair = Grammar::parse(Rule::city, "");
        assert!(pair.is_err());
    }

    #[test]
    #[should_panic]
    fn no_city_name() {
        let pair = Grammar::parse(Rule::city, ", 07400");
        assert!(pair.is_err());
    }

    #[test]
    fn no_city_zip_code_after_coma() {
        let pair = Grammar::parse(Rule::city, "Львів, ");
        assert!(pair.is_err());
    }

    #[test]
    fn no_city_zip_code() {
        let pair = Grammar::parse(Rule::city, "Львів");
        assert!(pair.is_err());
    }

    #[test]
    fn only_zip_code() {
        let pair = Grammar::parse(Rule::city, "00001");
        assert!(pair.is_err());
    }

    #[test]
    fn wrong_format_city() {
        let pair = Grammar::parse(Rule::city, "Київ. 00001");
        assert!(pair.is_err());
    }
}
