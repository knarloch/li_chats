use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2
    {
        println!("Usage: LiChats <file>");
        println!("Press enter to exit ");
        let _ = std::io::stdin().read(&mut [0u8]).unwrap();
        return Ok(());
    }
    let file = std::fs::File::open(&args[1])?;
    let file = std::io::BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file);
    let mut map = std::collections::HashMap::new();
    for result in rdr.records() {
        let record = result?;
        *map.entry(record[0].to_owned()).or_insert(0) += 1;
    }
    let mut conversations_with_single_msg = 0;
    let mut convestations_with_multiple_msg = 0;
    for conv in map {
        println!("{} : {}", conv.0, conv.1);
        if conv.1 == 1
        {
            conversations_with_single_msg += 1;
        } else { convestations_with_multiple_msg += 1; }
    }
    println!("");
    println!("Conversations with single message count: {}", conversations_with_single_msg);
    println!("Conversations with multiple message count: {}", convestations_with_multiple_msg);
    println!("Press enter to exit ");

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

    Ok(())
}
