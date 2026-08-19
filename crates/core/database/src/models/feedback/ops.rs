use revolt_result::Result;

use crate::Feedback;

#[cfg(feature = "mongodb")]
mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractFeedback: Sync + Send {
    /// Insert new feedback into the database
    async fn insert_feedback(&self, feedback: &Feedback) -> Result<()>;
}
