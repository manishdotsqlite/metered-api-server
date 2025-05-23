use api::{create_api_key, delete_api_key, perform_api_fn, validate_user};
use library::handle_rejection;
use warp::Filter;

mod sql;
mod api;
mod library;

#[tokio::main]
async fn  main() {
    let validate_user_route = warp::path!("validate" / String).and(warp::get())
    .and_then(|s: String| async move {
        match validate_user(&s).await {
            Ok(result) => Ok::<_, warp::Rejection>(warp::reply::json(&result)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    });

    let create_api_key = warp::path!("create-api-key" / String) .and(warp::get())
    .and_then(|s: String| async move {
        match create_api_key(&s).await {
            Ok(result) => {
                Ok::<_, warp::Rejection>(warp::reply::json(&result))},
            Err(e) => {
                Err(warp::reject::custom(e))},
        }
    });

    let delete_api_key = warp::path!("delete-api" / String / String).and(warp::post())
    .and_then(|username: String, api_key: String| async move {
        match delete_api_key(&username, &api_key).await {
            Ok(result) => Ok::<_, warp::Rejection>(warp::reply::json(&result)),
            Err(e) => Err(warp::reject::custom(e))
        }
    });

    let perform_api_function = warp::path!("add" / i32 / i32 / String / String).and(warp::get())
    .and_then(|a: i32, b: i32, username: String, api_key: String| async move {
        match perform_api_fn(a, b, &username, &api_key).await {
            Ok(result) => Ok::<_, warp::Rejection>(warp::reply::json(&result)),
            Err(e) => Err(warp::reject::custom(e))
        }
    });

   let routes = validate_user_route.or(create_api_key).or(delete_api_key).or(perform_api_function);
   warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
   
}
