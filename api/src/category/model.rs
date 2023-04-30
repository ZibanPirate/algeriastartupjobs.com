use serde::{Deserialize, Serialize};
use utility_types::pick;

#[pick(CompactCategory, [id, slug, name], [Serialize, Deserialize, Clone])]
#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub description: String,
}

// @TODO-ZM: write a Partial proc derive marco
pub struct PartialCategory {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub trait CategoryTrait {
    fn to_compact_category(&self) -> CompactCategory;
}

impl CategoryTrait for Category {
    fn to_compact_category(&self) -> CompactCategory {
        CompactCategory {
            id: self.id,
            slug: self.slug.clone(),
            name: self.name.clone(),
        }
    }
}
