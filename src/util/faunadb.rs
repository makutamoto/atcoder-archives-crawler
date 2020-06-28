// use std::error::Error;
// use std::fmt;

// use chrono::{Date as ChronoDate, Utc};
// use graphql_client::{GraphQLQuery, Response};

// use super::atcoder::User;

// type Date = String;

// #[derive(Debug)]
// enum FaunaDBError {
//     NoData,
// }

// impl fmt::Display for FaunaDBError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "FaunaDB: No Data")
//     }
// }

// impl Error for FaunaDBError { }

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "database/faunadb_schema.gql",
//     query_path = "database/create_rate.gql",
//     response_derives = "Debug",
// )]
// struct CreateRate;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "database/faunadb_schema.gql",
//     query_path = "database/register_users.gql",
//     response_derives = "Debug",
// )]
// struct RegisterUsers;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "database/faunadb_schema.gql",
//     query_path = "database/clear_crawling_flag.gql",
//     response_derives = "Debug",
// )]
// struct ClearCrawlingFlag;

// const FAUNADB_ENDPOINT: &str = "https://graphql.fauna.com/graphql";

// lazy_static! {
//     static ref FAUNADB_SECRET: String = std::env::var("FAUNADB_SECRET").unwrap();
// }

// pub fn create_rate(date: &ChronoDate<Utc>) -> Result<String, Box<dyn Error>> {
//     let date = date.format("%Y-%m-%d").to_string();
//     let request_body = CreateRate::build_query(create_rate::Variables{
//         date: date,
//     });
//     let client = reqwest::blocking::Client::new();
//     let res = client.post(FAUNADB_ENDPOINT)
//         .json(&request_body)
//         .bearer_auth(FAUNADB_SECRET.as_str())
//         .send()?;
//     let response_body: Response<create_rate::ResponseData> = res.json()?;
//     match response_body.data {
//         Some(data) => Ok(data.create_rates.id),
//         None => Err(Box::new(FaunaDBError::NoData)),
//     }
// }

// pub fn register_user(id: &String, provided_users: Vec<User>) -> Result<(), Box<dyn Error>> {
//     let mut users = Vec::new();
//     for user in provided_users {
//         users.push(register_users::UserInput{
//             name: user.name,
//             rate: user.rate,
//         });
//     }
//     let request_body = RegisterUsers::build_query(register_users::Variables{
//         id: (*id).clone(),
//         users: users,
//     });
//     let client = reqwest::blocking::Client::new();
//     let res = client.post(FAUNADB_ENDPOINT)
//         .json(&request_body)
//         .bearer_auth(FAUNADB_SECRET.as_str())
//         .send()?;
//     let response_body: Response<register_users::ResponseData> = res.json()?;
//     match response_body.data {
//         Some(_) => Ok(()),
//         None => Err(Box::new(FaunaDBError::NoData)),
//     }
// }

// pub fn clear_crawling_flag(id: &String) -> Result<(), Box<dyn Error>> {
//     let request_body = ClearCrawlingFlag::build_query(clear_crawling_flag::Variables{
//         id: (*id).clone(),
//     });
//     let client = reqwest::blocking::Client::new();
//     let res = client.post(FAUNADB_ENDPOINT)
//         .json(&request_body)
//         .bearer_auth(FAUNADB_SECRET.as_str())
//         .send()?;
//     let response_body: Response<clear_crawling_flag::ResponseData> = res.json()?;
//     match response_body.data {
//         Some(_) => Ok(()),
//         None => Err(Box::new(FaunaDBError::NoData)),
//     }
// }
