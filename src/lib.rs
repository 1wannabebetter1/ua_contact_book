use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug)]
pub struct Record {
    name: String,
    city: String,
    birth_date: String,
    phone_numbers: Vec<String>,
    email: String,
}

pub fn generate_contact() -> anyhow::Result<Vec<String>> {
    let unparsed_file = fs::read_to_string("test.txt").expect("cannot read file");
    let pairs = Grammar::parse(Rule::file, &unparsed_file)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    let mut contacts: Vec<String> = Vec::new();
    for pair in pairs.into_inner() {
        if pair.as_rule() == Rule::contact {
            contacts.push(pair.as_str().trim().to_string());
        }
    }
    Ok(contacts)
}

pub fn contact_to_record(user_contact: &str) -> anyhow::Result<Record> {
    let pairs = Grammar::parse(Rule::contact, user_contact)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    let mut name = String::new();
    let mut city = String::new();
    let mut birth_date = String::new();
    let mut phone_numbers = Vec::new();
    let mut email = String::new();
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::name => name = pair.as_str().trim().to_string(),
            Rule::city => city = pair.as_str().trim().to_string(),
            Rule::birth_date => birth_date = pair.as_str().trim().to_string(),
            Rule::numbers => {
                phone_numbers = pair
                    .into_inner()
                    .map(|p| p.as_str().trim().to_string())
                    .collect();
            }
            Rule::email => email = pair.as_str().trim().to_string(),
            _ => {}
        }
    }
    Ok(Record {
        name,
        city,
        birth_date,
        phone_numbers,
        email,
    })
}

pub fn generate_json(contacts: Vec<Record>) -> anyhow::Result<String> {
    let start: String = "[".to_string();
    let mut result: String = "".to_string();
    let mut is_first = true;
    for contact in contacts.iter() {
        let json: String = generate_single_record(contact)?;
        if !is_first {
            result = format!("{},\n{}", result, json);
        } else {
            result = format!("{}\n{}", result, json);
            is_first = false;
        }
    }
    let end: String = "\n]".to_string();
    result = format!("{}{}{}", start, result, end);
    Ok(result)
}

pub fn generate_single_record(contact: &Record) -> anyhow::Result<String> {
    let formatted_contact = format!(
        "\t{{\n\
         \t\t\"full_name\": \"{}\",\n\
         \t\t\"city\": \"{}\",\n\
         \t\t\"birth_date\": \"{}\",\n\
         \t\t\"contacts\": {{\n\
         \t\t\t\"phone_numbers\": {:?},\n\
         \t\t\t\"email\": \"{}\"\n\
         \t\t}}\n\
         \t}}",
        contact.name, contact.city, contact.birth_date, contact.phone_numbers, contact.email
    );

    Ok(formatted_contact)
}
