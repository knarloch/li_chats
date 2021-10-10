use std::io::prelude::*;
use std::collections::HashMap;
use chrono::{NaiveDateTime, DateTime, Utc};


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4
    {
        println!("Usage: LiChats <file> <since> <until>");
        println!("date format is ISO8601 rfc3339, i.e.: 1996-12-19T16:39:57-08:00");
        println!("Press enter to exit ");
        let _ = std::io::stdin().read(&mut [0u8]).unwrap();
        return Ok(());
    }

    let since = match DateTime::parse_from_rfc3339(&args[2]) {
        Ok(since) => since,
        Err(_) => panic!("Since: Expected date format is ISO8601 rfc3339, i.e.: 1996-12-19T16:39:57-08:00")
    };

    let until = match DateTime::parse_from_rfc3339(&args[3]) {
        Ok(since) => since,
        Err(_) => panic!("Until: Expected date format is ISO8601 rfc3339, i.e.: 1996-12-19T16:39:57-08:00")
    };


    let mut reader = csv::ReaderBuilder::new().flexible(true).from_path(&args[1])?;

    let mut map = HashMap::new();
    for result in reader.records() {
        let record = result?;
        match NaiveDateTime::parse_from_str(&record[5], "%Y-%m-%d %H:%M:%S UTC") {
            Ok(t) => {
                let t_utc = DateTime::<Utc>::from_utc(t, Utc);
                if since <= t_utc && t_utc <= until { *map.entry(record[0].to_owned()).or_insert(0) += 1 }
            }
            Err(e) => panic!("error parsing record: {:?} with {:?}", record, e)
        }
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
