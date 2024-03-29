
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}


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

/*
fn main() {
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
*/

use warp::Filter;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );
    println!("{:?}", question);

    let hello = warp::get()
        .map(|| format!("{}", "Hello, World!"));
    warp::serve(hello)
        .run(([127, 0, 0, 1], 1337))
        .await;

    let hello = warp::path("hello")
        .and(warp::path::param())
        .map(|name: String| format!("Hello, {}!", name));
    warp::serve(hello)
        .run(([127, 0, 0, 1], 1337))
        .await;

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}