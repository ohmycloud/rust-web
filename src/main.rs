use serde::Serialize;

#[derive(Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}


#[derive(Serialize)]
struct QuestionId(String); // New type pattern

impl Question {
    fn new(id: QuestionId,
           title: String,
           content: String,
           tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags
        }
    }
}

struct Position {
    longitude: f32,
    latitude: f32,
}

use std::fmt;
use std::fmt::Formatter;
use std::io::ErrorKind;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl fmt::Display for QuestionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "idl: {}", self.0)
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl fmt::Debug for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tags)
    }
}

use std::str::FromStr;
use std::io::{Error};

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id provided")
            )
        }
    }
}

use warp::{
    Filter,
    http::Method,
    filters::{cors::CorsForbidden,},
    reject::Reject,
    Rejection,
    Reply,
    http::StatusCode
};

#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}

pub async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(&question))
        }
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            "Cors Forbidden",
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(_InvalidId) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_methods(&[
           Method::PUT, Method::DELETE, Method::GET, Method::POST
        ]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
    Ok(())
}

#[test]
fn it_works() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "第一个问题".into(),
        "问题描述".into(),
        Some(vec!["raku".into(), "Grammar".into()])
    );
    println!("{:?}", question);

    assert_eq!("(1.987, 2.983)", format!("{}", Position {
        longitude: 1.987,
        latitude: 02.9830
    }));
}