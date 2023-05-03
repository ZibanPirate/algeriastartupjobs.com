use super::model::{Category, PartialCategory, PartialCategoryTrait};
use titlecase::titlecase;

pub fn generate_one_category_mock(category_id: i32) -> Category {
  Category {
    id: category_id,
    slug: format!("category_{}", category_id),
    name: format!("Category {}", category_id),
    description: format!("Description for category {}", category_id),
  }
}

pub fn generate_many_category_mocks_with_overwrite<F>(
  from: i32,
  to: i32,
  overwrite: Option<F>,
) -> Vec<Category>
where
  F: Fn(i32) -> PartialCategory,
{
  let mut categories: Vec<Category> = Vec::new();
  for i in from..to {
    let category = match overwrite {
      Some(ref f) => {
        let partial_category = f(i);
        let default_category = generate_one_category_mock(i);
        partial_category.to_category(default_category)
      }
      None => generate_one_category_mock(i),
    };
    categories.push(category);
  }
  categories
}

pub fn generate_many_category_mocks(from: i32, to: i32) -> Vec<Category> {
  generate_many_category_mocks_with_overwrite(
    from,
    to,
    Some(|_id| PartialCategory {
      id: None,
      slug: None,
      name: None,
      description: None,
    }),
  )
}

pub fn generate_categories_seed() -> Vec<Category> {
  let job_categories = [
    "Accounting/Finance",
    "Administrative",
    "Arts/Entertainment/Publishing",
    "Banking/Loans",
    "Construction/Facilities",
    "Customer Service",
    "Education/Training",
    "Engineering/Architecture",
    "Government/Military",
    "Healthcare",
    "Hospitality/Travel",
    "Human Resources",
    "Installation/Maintenance",
    "Insurance",
    "Internet",
    "Law Enforcement/Security",
    "Legal",
    "Management/Executive",
    "Manufacturing/Operations",
    "Marketing",
    "Non-Profit/Volunteer",
    "Other",
    "Product Management",
    "Project/Program Management",
    "Public Relations",
    "Quality Assurance",
    "Real Estate",
    "Research",
    "Sales",
    "Science/Technology",
    "Software Engineering",
    "Supply Chain/Logistics",
    "Training",
    "Writing/Editing",
  ];

  let total_categories_len = job_categories.len();
  generate_many_category_mocks_with_overwrite(
    0,
    total_categories_len as i32,
    Some(|id| {
      let category_name = job_categories[id as usize];
      PartialCategory {
        id: None,
        slug: Some(
          category_name
            .to_string()
            .replace("/", "_and_")
            .replace(" ", "_")
            .to_lowercase(),
        ),
        name: Some(titlecase(category_name)),
        description: Some(format!("All jobs related to {}", category_name)),
      }
    }),
  )
}
