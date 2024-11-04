use ua_contact_book::*;

fn main() -> anyhow::Result<()> {
    let contacts: Vec<String> = generate_contact()?;
    let mut records: Vec<Record> = Vec::new();
    for contact in contacts.iter() {
        let rec: Record = contact_to_record(contact)?;
        records.push(rec);
    }
    let result: String = generate_json(records)?;
    println!("{}", result);
    Ok(())
}
