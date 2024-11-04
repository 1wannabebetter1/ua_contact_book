use anyhow::anyhow;
use pest::Parser;
use ua_contact_book::*;

#[cfg(test)]
mod contact_test {
    use super::*;
    #[test]
    fn contact_creation_test() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;+380904652439, 0904652439; user@exam.pl")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;+380904652439, 0904652439; user@exam.pl");

        Ok(())
    }

    #[test]
    fn no_value_contact() {
        let pair = Grammar::parse(Rule::contact, "");
        assert!(pair.is_err());
    }

    #[test]
    fn missing_one_element_contact() {
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ;;06/06/2006;+380904652439, 0904652439; user@exam.pl");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::contact, "; Бровари, 07400;06/06/2006;+380904652439, 0904652439; user@exam.pl");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ; Бровари, 07400;;+380904652439, 0904652439; user@exam.pl");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;; user@exam.pl");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;+380904652439, 0904652439;");
        assert!(pair.is_err());
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ;;06/06/2006;; user@exam.pl");
        assert!(pair.is_err());
    }

    #[test]
    fn wrong_separator_contact() {
        let pair = Grammar::parse(Rule::contact, "Мар'яна ЄҐЇ: Бровари, 07400:06/06/2006:+380904652439, 0904652439:user@exam.pl");
        assert!(pair.is_err());
    }

    #[test]
    fn wrong_element_contact() {
        let pair = Grammar::parse(Rule::contact, "Мар'ян~а ЄҐЇ; Бро~вари, 07400;0~6/06/2006;+380904~652439, 0904652439; us~er@exam.pl");
        assert!(pair.is_err());
    }
}
