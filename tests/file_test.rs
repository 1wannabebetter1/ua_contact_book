use anyhow::anyhow;
use pest::Parser;
use ua_contact_book::*;

#[cfg(test)]
mod file_test {
    use super::*;
    #[test]
    fn file_creation_test() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::file, "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;+380904652439, 0904652439; user@exam.pl\nОлексій Ігорович; Львів, 79000; 12/04/1985; +380961234567, 0951234567; oleksii@example.com")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;+380904652439, 0904652439; user@exam.pl\nОлексій Ігорович; Львів, 79000; 12/04/1985; +380961234567, 0951234567; oleksii@example.com");

        Ok(())
    }

    #[test]
    fn missing_element_second() {
        let pair = Grammar::parse(Rule::file, "Мар'яна ЄҐЇ; Бровари, 07400;06/06/2006;+380904652439, 0904652439; user@exam.pl\nОлексій Ігорович;; 12/04/1985; +380961234567, 0951234567; oleksii@example.com");
        assert!(pair.is_err());
    }

    #[test]
    #[should_panic]
    fn no_value_file() {
        let pair = Grammar::parse(Rule::file, "");
        assert!(pair.is_err());
    }

    #[test]
    fn missing_element_of_contact() {
        let pair = Grammar::parse(Rule::file, "Мар'яна ЄҐЇ; Бровари, 07400;;+380904652439, 0904652439; user@exam.pl\nОлексій Ігорович; Львів, 79000; 12/04/1985;; oleksii@example.com");
        assert!(pair.is_err());
    }
}
