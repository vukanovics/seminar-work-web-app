use rocket::{form::Form, get, http::CookieJar, post, FromForm, State};
use rocket_dyn_templates::{handlebars::html_escape, Template};
use serde::{self, Serialize};

use crate::{
    application::{BaseLayoutContext, Error, ErrorResponder, SharedState},
    models::NewPost,
};

#[derive(Serialize, Debug)]
struct NewPostLayoutContext {
    #[serde(flatten)]
    base_context: BaseLayoutContext,
    previous_title: String,
    previous_description: String,
    previous_content: String,

    error: Option<String>,
    success: Option<String>,
}

impl NewPostLayoutContext {
    pub fn new(state: &State<SharedState>, jar: &CookieJar) -> Result<NewPostLayoutContext, Error> {
        Ok(NewPostLayoutContext {
            base_context: BaseLayoutContext::new(state, jar)?,
            previous_title: String::default(),
            previous_description: String::default(),
            previous_content: String::default(),
            error: None,
            success: None,
        })
    }

    pub fn with_previous_data(mut self, data: &NewPostForm) -> Self {
        self.previous_title = rocket_dyn_templates::handlebars::html_escape(&data.title);
        self.previous_content = rocket_dyn_templates::handlebars::html_escape(&data.content);
        self.previous_description =
            rocket_dyn_templates::handlebars::html_escape(&data.description);
        self
    }

    pub fn with_error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }

    pub fn with_success(mut self, success: Option<String>) -> Self {
        self.success = success;
        self
    }
}

#[get("/new_post")]
pub fn get(state: &State<SharedState>, jar: &CookieJar) -> Result<Template, ErrorResponder> {
    Ok(Template::render(
        "new_post",
        NewPostLayoutContext::new(state, jar)?,
    ))
}

#[derive(FromForm)]
#[allow(clippy::module_name_repetitions)]
pub struct NewPostForm {
    pub title: String,
    pub description: String,
    pub content: String,
}

impl NewPostForm {
    pub fn all_fields_populated(&self) -> bool {
        !self.title.is_empty() && !self.description.is_empty() && !self.content.is_empty()
    }
}

#[allow(clippy::needless_pass_by_value)]
#[post("/new_post", data = "<data>")]
pub fn post(
    jar: &CookieJar,
    state: &State<SharedState>,
    data: Form<NewPostForm>,
) -> Result<Template, ErrorResponder> {
    if let Some(error_message) = 'requirements: {
        if !data.all_fields_populated() {
            break 'requirements Some("All fields are required!");
        }

        let Some(user_info) = state.lock().unwrap().get_valid_user_info(jar)? else {
            break 'requirements Some("You need to log in first!")
        };

        let safe_title = html_escape(&data.title);
        let safe_description = html_escape(&data.description);
        let safe_content = html_escape(&data.content);

        state.lock().unwrap().database().create_post(NewPost {
            author: user_info.id,
            created_on: chrono::offset::Utc::now().naive_utc(),
            title: &safe_title,
            description: &safe_description,
            content: &safe_content,
        })?;

        None
    } {
        return Ok(Template::render(
            "new_post",
            NewPostLayoutContext::new(state, jar)?
                .with_previous_data(&data)
                .with_error(Some(error_message.to_owned())),
        ));
    }

    Ok(Template::render(
        "new_post",
        NewPostLayoutContext::new(state, jar)?
            .with_previous_data(&data)
            .with_success(Some("Created a new post!".to_string())),
    ))
}
