use rocket::{get, http::CookieJar, State};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    application::{BaseLayoutContext, Error, ErrorResponder, SharedState},
    models::Post,
};

#[derive(Serialize, Debug)]
struct ShortPostData {
    author: String,
    // human-readable creation time
    human_creation_time: String,
    title: String,
    description: String,
}

impl ShortPostData {
    fn from_post(state: &State<SharedState>, post: Post) -> Result<Self, Error> {
        let author = state
            .lock()
            .unwrap()
            .database()
            .get_user_by_id(post.author)?
            .ok_or(Error::PostHasInvalidUserId)?
            .username;
        let human_creation_time = post.created_on.format("%Y-%m%d %H:%M:%S").to_string();
        Ok(Self {
            author,
            title: post.title,
            description: post.description,
            human_creation_time,
        })
    }
}

#[derive(Serialize, Debug)]
struct IndexLayoutContext {
    #[serde(flatten)]
    base_context: BaseLayoutContext,

    posts: Vec<ShortPostData>,
}

impl IndexLayoutContext {
    pub fn new(state: &State<SharedState>, jar: &CookieJar) -> Result<IndexLayoutContext, Error> {
        let posts = state.lock().unwrap().database().get_latest_x_posts(10)?;
        // top posts drops to drop the mutex lock, else we are stuck
        let posts = posts
            .into_iter()
            .map(|post| ShortPostData::from_post(state, post))
            .collect::<Result<Vec<ShortPostData>, Error>>()?;
        Ok(IndexLayoutContext {
            base_context: BaseLayoutContext::new(state, jar)?,
            posts,
        })
    }
}

#[get("/")]
pub fn get(jar: &CookieJar, state: &State<SharedState>) -> Result<Template, ErrorResponder> {
    let context = IndexLayoutContext::new(state, jar)?;
    Ok(Template::render("index", context))
}
