use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
struct QuestionId(String); // New type pattern

#[derive(Debug)]
struct Position {
    longitude: f32,
    latitude: f32,
}

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions/questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
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

pub async fn add_question(
    store: Store,
    question: Question
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::OK,
    ))
}

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect();
        let res = &res[pagination.start .. pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

pub fn extract_pagination(
    params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(
            Pagination {
                start: params.get("start").unwrap().parse::<usize>()
                    .map_err(Error::ParseError)?,
                end: params.get("end").unwrap().parse::<usize>()
                    .map_err(Error::ParseError)?,
            });
    }
    Err(Error::MissingParameters)
}

impl Reject for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            },
            Error::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_methods(&[
           Method::PUT, Method::DELETE, Method::GET, Method::POST
        ]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let routes = get_questions
        .or(add_question)
        .with(cors)
        .recover(return_error);

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

    assert_eq!("(1.987, 2.983)", format!("{:?}", Position {
        longitude: 1.987,
        latitude: 02.9830
    }));
}