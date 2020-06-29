use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use std::{thread, time};

// use chrono::Utc;

use crate::util::{atcoder, faunadb};
use atcoder::Contest;

lazy_static! {
    static ref ONE_SECONDS: time::Duration = time::Duration::from_secs(1); 
}

fn take_break() {
    print!("Taking a break");
    for _ in 0..10 {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(*ONE_SECONDS);
    }
    println!();
}

fn crawl_page(contests: &mut HashMap<String, Contest>, page: u32) -> Result<(), Box<dyn Error>> {
    let user_names = atcoder::get_user_list(page)?;
    println!("Got {} user(s).", user_names.len());
    take_break();
    for name in user_names {
        println!("Crawling {}...", name);
        let history = atcoder::get_user_history(&name)?;
        atcoder::assign_user(contests, &name, &history);
        take_break();
    }
    Ok(())
}

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
//         // faunadb::register_user(&id, users)?
//         page += 1;
//     }
//     // faunadb::clear_crawling_flag(&id)?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_break() {
        let now = time::Instant::now();
        take_break();
        assert_eq!(now.elapsed().as_secs(), 10);
    }

    #[test]
    fn test_crawl_page() {
        let mut contests = HashMap::new();
        crawl_page(&mut contests, 1);
        println!("{:?}", contests);
    }
}
