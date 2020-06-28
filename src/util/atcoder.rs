use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;

use scraper::{ Html, Selector };
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct UserHistory {
    #[serde(rename = "IsRated")]
    is_rated: bool,
    #[serde(rename = "NewRating")]
    new_rating: u32,
    #[serde(rename = "ContestScreenName")]
    contest_screen_name: String,
    #[serde(rename = "EndTime")]
    end_time: String,
}

#[derive(Debug)]
struct Contest {
    date: String,
    ranking: HashMap<String, u32>,
}

impl UserHistory {
    fn new(is_rated: bool, new_rating: u32, contest_screen_name: &str, end_time: &str) -> UserHistory {
        let contest_screen_name = String::from(contest_screen_name);
        let end_time = String::from(end_time);
        UserHistory { is_rated, new_rating, contest_screen_name, end_time }
    }
}

lazy_static! {
    static ref SELECTOR_USER: Selector = Selector::parse("#main-container .table-responsive tbody tr").unwrap();
    static ref SELECTOR_USER_NAME: Selector = Selector::parse(".username").unwrap();
    static ref SELECTOR_USER_RATE: Selector = Selector::parse("td:nth-of-type(4)").unwrap();
}

fn get_user_history(name: &str) -> Result<Vec<UserHistory>, Box<dyn Error>> {
    let link = format!("https://atcoder.jp/users/{}/history/json", name);
    let res = reqwest::blocking::get(link.as_str())?;
    let json = res.text()?;
    let history: Vec<UserHistory> = serde_json::from_str(&json)?;
    Ok(history)
}

fn assign_user(contests: &mut HashMap<String, Contest>, name: &str, history: &Vec<UserHistory>) {
    let name = String::from(name);
    for contest_data in history {
        if contest_data.is_rated {
            let screen_name = contest_data.contest_screen_name.clone();
            if !contests.contains_key(&screen_name) {
                let date = contest_data.end_time.clone();
                contests.insert(screen_name.clone(), Contest{ date, ranking: HashMap::new() });
            }
            let contest = contests.get_mut(&screen_name).unwrap();
            contest.ranking.insert(name.clone(), contest_data.new_rating);
        }
    }
}

// pub fn get_rates(page: u32) -> Result<Vec<User>, Box<dyn Error>> {
//     let mut result = Vec::new();
//     let link = format!("https://atcoder.jp/ranking?page={}", page);
//     let res = reqwest::blocking::get(link.as_str())?;
//     let html = res.text()?;
//     let document = Html::parse_document(html.as_str());
//     let users = document.select(&SELECTOR_USER);
//     for user in users {
//         let name = user.select(&SELECTOR_USER_NAME).next();
//         let rate = user.select(&SELECTOR_USER_RATE).next();
//         if let (Some(name), Some(rate)) = (name, rate) {
//             let name = name.text().next();
//             let rate = rate.text().next();
//             if let (Some(name), Some(rate)) = (name, rate) {
//                 let name = String::from_str(name);
//                 let rate = rate.parse();
//                 if rate.is_err() {
//                     eprintln!("Rate is not a number. User: {:?}", name);
//                 }
//                 if let (Ok(name), Ok(rate)) = (name, rate) {
//                     result.push(User{ name: name, rate: rate });
//                 }
//             }
//         }
//     }
//     Ok(result)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_history() {
        let history = get_user_history("tourist").unwrap();
        assert_eq!(history[0].is_rated, true);
        assert_eq!(history[0].new_rating, 2720);
        assert_eq!(history[0].contest_screen_name, String::from("agc004.contest.atcoder.jp"));
        assert_eq!(history[0].end_time, String::from("2016-09-04T22:50:00+09:00"));
    }

    #[test]
    fn test_assign_user() {
        let mut contests = HashMap::new();
        let history1 = vec![
            UserHistory::new(true, 9999, "abc001", "2016-09-04T22:50:00+09:00"),
            UserHistory::new(false, 9999, "abc002", "2016-09-05T22:50:00+09:00"),
        ];
        let history2 = vec![
            UserHistory::new(true, 9, "abc001", "2016-09-04T22:50:00+09:00"),
            UserHistory::new(false, 9, "abc002", "2016-09-05T22:50:00+09:00" ),
        ];
        assign_user(&mut contests, "tourist", &history1);
        assign_user(&mut contests, "Makutamoto", &history2);
        
        assert!(contests.contains_key("abc001"));
        assert!(!contests.contains_key("abc002"));
        assert_eq!(contests["abc001"].date, "2016-09-04T22:50:00+09:00");
        assert_eq!(contests["abc001"].ranking, hashmap!{String::from("tourist") => 9999, String::from("Makutamoto") => 9});
    }
}
