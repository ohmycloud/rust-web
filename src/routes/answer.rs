use crate::store::Store;
use crate::types::answer::NewAnswer;
use warp::http::StatusCode;
use crate::profanity::check_profanity;

pub async fn add_answer(
    store: Store,
    answer: NewAnswer,
) -> Result<impl warp::reply::Reply, warp::Rejection> {
    let content = match
        check_profanity(answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let answer = NewAnswer {
        content,
        question_id: answer.question_id,
    };
    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
