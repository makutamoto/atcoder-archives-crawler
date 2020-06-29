use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use std::{thread, time};

use crate::util::atcoder;
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

fn crawl_page(contests: &mut HashMap<String, Contest>, page: u32) -> Result<bool, Box<dyn Error>> {
    let user_names = atcoder::get_user_list(page)?;
    if user_names.is_empty() {
        return Ok(false);
    }
    println!("Got {} user(s).", user_names.len());
    take_break();
    for (i, name) in user_names.iter().enumerate() {
        println!("[{}: {}/{}] Crawling {}...", page, i + 1, user_names.len(), name);
        let history = atcoder::get_user_history(&name)?;
        atcoder::assign_user(contests, &name, &history);
        take_break();
    }
    Ok(true)
}

pub fn crawl(contests: &mut HashMap<String, Contest>) -> Result<(), Box<dyn Error>> {
    let mut page = 1;
    loop {
        println!("Crawling a page {}...", page);
        let next = crawl_page(contests, page)?;
        take_break();
        if !next {
            break;
        }
        page += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_break() {
        let now = time::Instant::now();
        take_break();
        assert_eq!(now.elapsed().as_secs(), 10);
    }
}
