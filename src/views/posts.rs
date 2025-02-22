use crate::resources::posts::Post;
use crate::schemas::posts::{CreatePostRequest, PostResponse, UpdatePostRequest};
use actix_web::web::{Data, Json, Path};
use actix_web::{error::ErrorInternalServerError, HttpResponse};
use apistos::{api_operation, web};
use surrealdb::engine::any::Any;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(
                web::resource("")
                    .route(web::post().to(create_post))
                    .route(web::get().to(list_posts)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_post))
                    .route(web::put().to(update_post))
                    .route(web::delete().to(delete_post)),
            ),
    );
}

#[api_operation(summary = "Create a new post")]
async fn create_post(
    db: Data<Surreal<Db>>,
    post: Json<CreatePostRequest>,
) -> actix_web::Result<HttpResponse> {
    let create_post = crate::resources::posts::CreatePost::from(post.into_inner());
    let post = Post::new(create_post);
    let result = crate::resources::posts::create_post(&db, post)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(PostResponse::from(result)))
}

#[api_operation(summary = "Retrieve a post by ID")]
async fn get_post(db: Data<Surreal<Db>>, id: Path<String>) -> actix_web::Result<HttpResponse> {
    let post = crate::resources::posts::get_post(&db, &id)
        .await
        .map_err(ErrorInternalServerError)?;
    match post {
        Some(post) => Ok(HttpResponse::Ok().json(PostResponse::from(post))),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[api_operation(summary = "List all posts")]
async fn list_posts(db: Data<Surreal<Db>>) -> actix_web::Result<HttpResponse> {
    let posts = crate::resources::posts::list_posts(&db)
        .await
        .map_err(ErrorInternalServerError)?;
    let response: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

#[api_operation(summary = "Update a post by ID")]
async fn update_post(
    db: Data<Surreal<Db>>,
    id: Path<String>,
    update: Json<UpdatePostRequest>,
) -> actix_web::Result<HttpResponse> {
    let current_post = match crate::resources::posts::get_post(&db, &id).await {
        Ok(Some(post)) => post,
        Ok(None) => return Ok(HttpResponse::NotFound().finish()),
        Err(e) => return Err(ErrorInternalServerError(e)),
    };

    let updated_post = Post {
        title: update.title.clone().unwrap_or(current_post.title),
        content: update.content.clone().unwrap_or(current_post.content),
        published: update.published.unwrap_or(current_post.published),
        tags: update.tags.clone().unwrap_or(current_post.tags),
        ..current_post
    };

    let result = crate::resources::posts::update_post(&db, &id, updated_post)
        .await
        .map_err(ErrorInternalServerError)?;

    match result {
        Some(post) => Ok(HttpResponse::Ok().json(PostResponse::from(post))),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[api_operation(summary = "Delete a post by ID")]
async fn delete_post(db: Data<Surreal<Db>>, id: Path<String>) -> actix_web::Result<HttpResponse> {
    let result = crate::resources::posts::delete_post(&db, &id)
        .await
        .map_err(ErrorInternalServerError)?;
    match result {
        Some(post) => Ok(HttpResponse::Ok().json(PostResponse::from(post))),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
