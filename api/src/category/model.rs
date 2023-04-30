use serde::{Deserialize, Serialize};
use utility_types::{partial, pick};

#[pick(CompactCategory, [id, slug, name], [Serialize, Deserialize, Clone])]
#[partial(PartialCategory)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub description: String,
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

pub trait PartialCategoryTrait {
    fn to_category(&self, fallback_category: Category) -> Category;
}

impl PartialCategoryTrait for PartialCategory {
    fn to_category(&self, fallback_category: Category) -> Category {
        Category {
            id: self.id.unwrap_or(fallback_category.id),
            slug: self.slug.clone().unwrap_or(fallback_category.slug),
            name: self.name.clone().unwrap_or(fallback_category.name),
            description: self
                .description
                .clone()
                .unwrap_or(fallback_category.description),
        }
    }
}
