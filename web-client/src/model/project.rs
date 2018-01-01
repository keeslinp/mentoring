use chrono::{DateTime, Utc};

use model::{Comment, MaybeLoading, Model, User};

#[derive(Debug)]
pub struct Project {
    pub comments: Vec<Comment>,
    pub created: DateTime<Utc>,
    pub creator_id: usize,
    pub developer_id: Option<usize>,
    pub long_description: String,
    pub name: String,
    pub mentor_id: Option<usize>,
    pub short_description: String,
    pub tags: Vec<String>,
}

impl Project {
    pub fn creator<'a>(&self, model: &'a Model) -> Option<&'a User> {
        model.find_user(self.creator_id)
    }

    pub fn developer<'a>(&self, model: &'a Model) -> MaybeLoading<&'a User> {
        if let Some(developer_id) = self.developer_id {
            if let Some(user) = model.find_user(developer_id) {
                MaybeLoading::Present(user)
            } else {
                MaybeLoading::Loading
            }
        } else {
            MaybeLoading::NotPresent
        }
    }

    pub fn mentor<'a>(&self, model: &'a Model) -> MaybeLoading<&'a User> {
        if let Some(mentor_id) = self.mentor_id {
            if let Some(user) = model.find_user(mentor_id) {
                MaybeLoading::Present(user)
            } else {
                MaybeLoading::Loading
            }
        } else {
            MaybeLoading::NotPresent
        }
    }
}
