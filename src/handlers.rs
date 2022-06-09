use warp::{Rejection, Reply};

use crate::{Database, Todo};

pub async fn list_todos_handler(db: Database) -> Result<impl Reply, Rejection> {
  let db = db.lock().await;
  let todos = db
    .clone()
    .init_iter()
    .map(|(_, v)|v)
    .collect::<Vec<Todo>>();
  Ok(warp::reply::json(&todos))
}

pub async fn get_todos_handler(db: Database, id: u64) -> Result<impl Reply, Rejection> {
  let db = db.lock().await;
  let todo = db.get(&id);
  match todo {
    None => Err(warp::reject::not_found()),
    Some(u) => Ok(warp::reply::json(&u)),
  }
}

pub async fn pud_todos_handler(db: Database, id: u64, todo: Todo) -> Result<impl Reply, Rejection> {
  if id != user.id() {
    return Ok(warp::reply::with_status(
      warp::reply::json(&()),
      warp::http::StatusCode::BAD_REQUEST,
    ));
  }
  let mut db = db.lock().await;
  db.insert(user.id(), user.clone());
  Ok(warp::reply::with_status(
    warp::reply::json(&todo),
    warp::http::StatusCode::OK,
  ))
}