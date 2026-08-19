auto_derived!(
    /// Category of user-submitted feedback
    pub enum FeedbackCategory {
        Bug,
        Suggestion,
        Question,
        Other,
    }
);

auto_derived!(
    /// User-submitted feedback (AgenXs custom feature)
    pub struct Feedback {
        /// Unique Id
        #[serde(rename = "_id")]
        pub id: String,
        /// Id of the user who submitted the feedback
        pub author_id: String,
        /// Username of the author (denormalised for easy review)
        pub author_username: String,
        /// Category of feedback
        pub category: FeedbackCategory,
        /// Feedback content
        pub content: String,
        /// URL of the page where the feedback was submitted
        pub page_url: String,
        /// Browser user agent string
        pub user_agent: String,
        /// Whether the feedback has been resolved
        #[serde(default)]
        pub resolved: bool,
    }
);
