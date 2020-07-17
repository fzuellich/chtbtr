use serde::{Deserialize, Serialize};

use crate::types::{GerritUsername, PatchStatus, ProjectName};

#[derive(Serialize, Deserialize, Debug)]
pub enum GerritTrigger {
    CommentAdded(CommentAddedData),
    ReviewerAdded(ReviewerAddedData),
    PatchStatusChanged(PatchStatusChangedData),
}

// Always necessary to construct a meaningful message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BaseData {
    pub change_owner: String,
    pub change_owner_username: GerritUsername,
    pub change_url: String,
    pub project: ProjectName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchStatusChangedData {
    pub base: BaseData,
    pub author_username: GerritUsername,
    pub patch_status: PatchStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentAddedData {
    pub base: BaseData,
    pub author: String,
    pub author_username: GerritUsername,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewerAddedData {
    pub change_owner: String,
    pub change_owner_username: GerritUsername,
    pub reviewer: String,
    pub reviewer_username: GerritUsername,
    pub change_url: String,
    pub project: String,
}
