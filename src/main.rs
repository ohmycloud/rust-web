use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
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

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(_error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            "Cors Forbidden",
            StatusCode::FORBIDDEN,
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
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_methods(&[
           Method::PUT, Method::DELETE, Method::GET, Method::POST
        ]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
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

    assert_eq!("(1.987, 2.983)", format!("{:?}", Position {
        longitude: 1.987,
        latitude: 02.9830
    }));
}