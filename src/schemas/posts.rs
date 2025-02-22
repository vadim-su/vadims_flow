use crate::resources::posts::Post;
use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<String>>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id.map(|t| t.id.to_string()).unwrap_or_default(),
            title: post.title,
            content: post.content,
            author: post.author,
            published: post.published,
            created_at: post.created_at,
            updated_at: post.updated_at,
            tags: post.tags,
        }
    }
}

impl From<CreatePostRequest> for crate::resources::posts::CreatePost {
    fn from(req: CreatePostRequest) -> Self {
        Self {
            title: req.title,
            content: req.content,
            author: req.author,
            tags: req.tags,
        }
    }
}
