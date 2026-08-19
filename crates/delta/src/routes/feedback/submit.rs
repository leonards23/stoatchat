use revolt_database::{Database, Feedback, FeedbackCategory, User};
use revolt_result::{create_error, Result};
use rocket_empty::EmptyResponse;
use serde::Deserialize;
use ulid::Ulid;
use validator::Validate;

use rocket::{serde::json::Json, State};

/// # Feedback Data
#[derive(Validate, Deserialize, JsonSchema)]
pub struct DataSubmitFeedback {
    /// Category of the feedback
    category: FeedbackCategory,
    /// Feedback content
    #[validate(length(min = 1, max = 2000))]
    content: String,
    /// URL of the page where the feedback was submitted
    #[validate(length(min = 0, max = 500))]
    #[serde(default)]
    page_url: String,
    /// Browser user agent string
    #[validate(length(min = 0, max = 500))]
    #[serde(default)]
    user_agent: String,
}

/// # Submit Feedback
///
/// Submit feedback (bug, suggestion, question, other) to the instance administrators.
#[openapi(tag = "Feedback")]
#[post("/submit", data = "<data>")]
pub async fn submit(
    db: &State<Database>,
    user: User,
    data: Json<DataSubmitFeedback>,
) -> Result<EmptyResponse> {
    let data = data.into_inner();
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    // Bots cannot submit feedback
    if user.bot.is_some() {
        return Err(create_error!(IsBot));
    }

    let feedback = Feedback {
        id: Ulid::new().to_string(),
        author_id: user.id.clone(),
        author_username: user.username.clone(),
        category: data.category,
        content: data.content,
        page_url: data.page_url,
        user_agent: data.user_agent,
        resolved: false,
    };

    db.insert_feedback(&feedback).await?;

    Ok(EmptyResponse)
}
