use comrak::{markdown_to_html, ComrakOptions};
use rocket::{get, http::CookieJar, State};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    application::{BaseLayoutContext, Error, ErrorResponder, SharedState},
    models::Post,
};

#[derive(Serialize, Debug)]
struct FullPostData {
    author: String,
    human_readable_creation_time: String,
    title: String,
    description: String,
    content: String,
}

impl FullPostData {
    fn from_post<'a>(state: &State<SharedState>, post: Post) -> Result<Self, Error> {
        let author = state
            .lock()
            .unwrap()
            .database()
            .get_user_by_id(post.author)?
            .ok_or(Error::PostHasInvalidUserId)?
            .username;

        let human_readable_creation_time =
            post.created_on.format("%d. %m. %Y. %H:%M:%S").to_string();

        let content = markdown_to_html(&post.content, &ComrakOptions::default());

        Ok(Self {
            author,
            title: post.title,
            description: post.description,
            content,
            human_readable_creation_time,
        })
    }
}

#[derive(Serialize, Debug)]
struct PostLayoutContext {
    #[serde(flatten)]
    base_context: BaseLayoutContext,

    post: FullPostData,
}

impl PostLayoutContext {
    pub fn new(
        state: &State<SharedState>,
        jar: &CookieJar,
        id: i32,
    ) -> Result<PostLayoutContext, Error> {
        let post = state.lock().unwrap().database().get_post_by_id(id)?;
        let post = post
            .map(|post| FullPostData::from_post(state, post))
            .transpose()?;
        if let Some(post) = post {
            Ok(PostLayoutContext {
                base_context: BaseLayoutContext::new(state, jar)?,
                post,
            })
        } else {
            Err(Error::InvalidPostId)
        }
    }
}

#[get("/post/<id>")]
pub fn get(
    jar: &CookieJar,
    state: &State<SharedState>,
    id: i32,
) -> Result<Template, ErrorResponder> {
    let context = PostLayoutContext::new(state, jar, id)?;
    Ok(Template::render("post", context))
}
