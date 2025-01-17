// This file is just an example and isn't included by default.
// Use it as a building block for the rest of your worker-specific server functions.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostData {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
}

#[cfg_attr(feature = "ssr", worker::send)] // <- required to await data from env
#[server(GetPost)]
pub async fn get_post(post_id: i64) -> Result<Option<PostData>, ServerFnError> {
    use std::sync::Arc;
    use worker::*;
    use axum::Extension;
    use leptos_axum::extract;

    // Get our Worker env variable from axum
    let Extension(env): Extension<Arc<Env>> = extract().await?;

    // Connect to our database
    let d1 = env.d1("DB")?;

    // Load the post data
    let statement  = d1.prepare("SELECT * FROM post where id=?");
    let query = statement.bind(&[post_id.into()])?;
    let result: Option<PostData> = query.first::<PostData>(None).await.unwrap();

    Ok(result)
}
