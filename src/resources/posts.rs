use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub author: String,
    pub published: bool,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
}

impl Post {
    pub fn new(create_post: CreatePost) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title: create_post.title,
            content: create_post.content,
            author: create_post.author,
            published: false,
            created_at: now,
            updated_at: now,
            tags: create_post.tags,
        }
    }
}

pub async fn create_post(db: &Surreal<Db>, post: Post) -> surrealdb::Result<Post> {
    let result: Option<Post> = db.create("post").content(post).await?;
    Ok(result.unwrap())
}

pub async fn get_post(db: &Surreal<Db>, id: &str) -> surrealdb::Result<Option<Post>> {
    db.select(("post", id)).await
}

pub async fn list_posts(db: &Surreal<Db>) -> surrealdb::Result<Vec<Post>> {
    db.select("post").await
}

pub async fn update_post(
    db: &Surreal<Db>,
    id: &str,
    post: Post,
) -> surrealdb::Result<Option<Post>> {
    db.update(("post", id)).content(post).await
}

pub async fn delete_post(db: &Surreal<Db>, id: &str) -> surrealdb::Result<Option<Post>> {
    db.delete(("post", id)).await
}
