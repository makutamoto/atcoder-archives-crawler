// use std::error::Error;
// use std::io::{self, Write};
// use std::{thread, time};

// use chrono::Utc;

// use crate::util::{atcoder, faunadb};

// lazy_static! {
//     static ref ONE_SECONDS: time::Duration = time::Duration::from_secs(1); 
// }

// pub fn crawl() -> Result<(), Box<dyn Error>> {
//     let today = Utc::today();
//     // let id = faunadb::create_rate(&today)?;
//     let mut page = 1;
//     loop {
//         println!("Crawling a page {}...", page);
//         // let users = atcoder::get_rates(page)?;
//         println!("Got {} users.", users.len());
//         if users.is_empty() {
//             break;
//         }
//         // faunadb::register_user(&id, users)?;
//         print!("Taking a break");
//         for _ in 0..10 {
//             print!(".");
//             io::stdout().flush().unwrap();
//             thread::sleep(*ONE_SECONDS);
//         }
//         println!();
//         page += 1;
//     }
//     // faunadb::clear_crawling_flag(&id)?;
//     Ok(())
// }
