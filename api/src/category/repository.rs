use crate::utils::error::DataAccessError;

use super::{mocks::generate_categories_seed, model::Category};

pub struct CategoryRepository {}

impl CategoryRepository {
    pub fn get_many_categories_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> Result<Vec<Category>, DataAccessError> {
        let categories = generate_categories_seed();
        let mut result: Vec<Category> = Vec::new();
        for id in ids.iter() {
            let category = categories
                .iter()
                .find(|category| category.id == *id)
                .ok_or(DataAccessError::NotFound)?;
            result.push(category.clone());
        }
        Ok(result)
    }

    pub fn get_one_category_by_id(&self, id: i32) -> Result<Category, DataAccessError> {
        let categories = generate_categories_seed();
        let category = categories
            .iter()
            .find(|category| category.id == id)
            .ok_or(DataAccessError::NotFound)?;
        Ok(category.clone())
    }
}
