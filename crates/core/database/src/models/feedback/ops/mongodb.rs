use revolt_result::Result;

use crate::Feedback;
use crate::MongoDb;

use super::AbstractFeedback;

static COL: &str = "feedback";

#[async_trait]
impl AbstractFeedback for MongoDb {
    /// Insert new feedback into the database
    async fn insert_feedback(&self, feedback: &Feedback) -> Result<()> {
        query!(self, insert_one, COL, &feedback).map(|_| ())
    }
}
