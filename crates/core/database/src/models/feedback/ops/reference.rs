use revolt_result::Result;

use crate::Feedback;
use crate::ReferenceDb;

use super::AbstractFeedback;

#[async_trait]
impl AbstractFeedback for ReferenceDb {
    /// Insert new feedback into the database
    async fn insert_feedback(&self, feedback: &Feedback) -> Result<()> {
        let mut store = self.feedback.lock().await;
        if store.contains_key(&feedback.id) {
            Err(create_database_error!("insert", "feedback"))
        } else {
            store.insert(feedback.id.to_string(), feedback.clone());
            Ok(())
        }
    }
}
