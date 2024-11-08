use ua_contact_book::*;

fn main() -> anyhow::Result<()> {
    
    let result: String = parse("tesst.txt")?;
    println!("{}", result);

    Ok(())
}
