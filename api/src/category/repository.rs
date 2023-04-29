use super::{mocks::generate_categories_seed, model::Category};

pub fn get_category_by_id(category_id: i32) -> Result<Category, ()> {
    Ok(generate_categories_seed()
        .get(category_id as usize)
        .unwrap()
        .clone())
}
