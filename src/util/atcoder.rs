use std::error::Error;
use std::collections::BTreeMap;

use scraper::{ Html, Selector };
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserHistory {
    #[serde(rename = "IsRated")]
    is_rated: bool,
    #[serde(rename = "NewRating")]
    new_rating: u32,
    #[serde(rename = "ContestScreenName")]
    contest_screen_name: String,
    #[serde(rename = "ContestName")]
    contest_name: String,
    #[serde(rename = "EndTime")]
    end_time: String,
}

#[derive(Debug, PartialEq)]
pub struct User {
    rating: u32,
    end_time: String,
}

#[derive(Debug)]
pub struct Contest {
    name: String,
    date: String,
    ranking: BTreeMap<String, User>,
}

impl UserHistory {
    #[cfg(test)]
    fn new(is_rated: bool, new_rating: u32, contest_screen_name: &str, contest_name: &str, end_time: &str) -> UserHistory {
        let contest_screen_name = String::from(contest_screen_name);
        let contest_name = String::from(contest_name);
        let end_time = String::from(end_time);
        UserHistory { is_rated, new_rating, contest_screen_name, contest_name, end_time }
    }
}

lazy_static! {
    static ref SELECTOR_USER_NAME: Selector = Selector::parse("#main-container .table-responsive tbody tr .username").unwrap();
}

pub fn get_user_history(name: &str) -> Result<Vec<UserHistory>, Box<dyn Error>> {
    let link = format!("https://atcoder.jp/users/{}/history/json", name);
    let res = reqwest::blocking::get(link.as_str())?;
    let json = res.text()?;
    let history: Vec<UserHistory> = serde_json::from_str(&json)?;
    Ok(history)
}

pub fn assign_user(contests: &mut BTreeMap<String, Contest>, name: &str, history: &Vec<UserHistory>) {
    let name = String::from(name);
    for contest_data in history {
        if contest_data.is_rated {
            let screen_name = contest_data.contest_screen_name.clone();
            let contest_name = contest_data.contest_name.clone();
            if !contests.contains_key(&screen_name) {
                let date = contest_data.end_time.clone();
                contests.insert(screen_name.clone(), Contest{ name: contest_name, date, ranking: BTreeMap::new() });
            }
            let contest = contests.get_mut(&screen_name).unwrap();
            let user = User{
                rating: contest_data.new_rating,
                end_time: contest_data.end_time.clone(),
            };
            contest.ranking.insert(name.clone(), user);
        }
    }
}

pub fn get_user_list(page: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let link = format!("https://atcoder.jp/ranking/all?page={}", page);
    let res = reqwest::blocking::get(link.as_str())?;
    let html = res.text()?;
    let document = Html::parse_document(html.as_str());
    let user_names = document.select(&SELECTOR_USER_NAME);
    Ok(user_names.map(|name| String::from(name.text().next().unwrap())).collect())
}

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
        let mut contests = BTreeMap::new();
        let history1 = vec![
            UserHistory::new(true, 9999, "abc001", "AtCoder Beginner Contest 001", "2016-09-04T22:50:00+09:00"),
            UserHistory::new(false, 9999, "abc002", "AtCoder Beginner Contest 002", "2016-09-05T22:50:00+09:00"),
        ];
        let history2 = vec![
            UserHistory::new(true, 9, "abc001", "AtCoder Beginner Contest 001", "2016-09-04T22:50:00+09:00"),
            UserHistory::new(false, 9, "abc002", "AtCoder Beginner Contest 002", "2016-09-05T22:50:00+09:00" ),
        ];
        assign_user(&mut contests, "tourist", &history1);
        assign_user(&mut contests, "Makutamoto", &history2);
        
        assert!(contests.contains_key("abc001"));
        assert!(!contests.contains_key("abc002"));
        assert_eq!(contests["abc001"].name, "AtCoder Beginner Contest 001");
        assert_eq!(contests["abc001"].date, "2016-09-04T22:50:00+09:00");
        assert_eq!(contests["abc001"].ranking,
            btreemap!{
                String::from("tourist") => User{ rating: 9999, end_time: String::from("2016-09-04T22:50:00+09:00") },
                String::from("Makutamoto") => User{ rating: 9, end_time: String::from("2016-09-04T22:50:00+09:00") },
            }
        );
    }

    #[test]
    fn test_get_user_list() {
        let list = get_user_list(1).unwrap();
        assert_eq!(list.len(), 100);
    }
}
