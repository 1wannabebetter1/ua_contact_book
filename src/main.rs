use ua_contact_book::*;

fn help() {
    println!("Usage: cargo run <command>");
    println!("Commands:");
    println!("  help       Show help information");
    println!("  credits    Show credits");
    println!("  <path>     Parse file with required structure to json form. You can use test.txt for example");
}

fn credits() {
    println!("ua_contact_book - Created by Vladyslav Vitisk");
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"help".to_string()) || args.len() != 2 {
        help()
    } else if args.contains(&"credits".to_string()) {
        credits()
    } else {
        let result: String = parse(&args[1])?;
        println!("{}", result);
    }
    Ok(())
}
