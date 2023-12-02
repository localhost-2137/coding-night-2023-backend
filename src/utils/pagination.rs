use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub skip: Option<u32>,
    pub take: Option<u32>,
}