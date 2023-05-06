use super::model::{PartialTag, PartialTagTrait, Tag};
use titlecase::titlecase;

pub fn generate_one_tag_mock(tag_id: u32) -> Tag {
  Tag {
    id: tag_id,
    slug: format!("tag_{}", tag_id),
    name: format!("Tag {}", tag_id),
  }
}

pub fn generate_many_tag_mocks_with_overwrite<F>(
  from: u32,
  to: u32,
  overwrite: Option<F>,
) -> Vec<Tag>
where
  F: Fn(u32) -> PartialTag,
{
  let mut tags: Vec<Tag> = Vec::new();
  for i in from..to {
    let tag = match overwrite {
      Some(ref f) => {
        let partial_tag = f(i);
        let default_tag = generate_one_tag_mock(i);
        partial_tag.to_tag(default_tag)
      }
      None => generate_one_tag_mock(i),
    };
    tags.push(tag);
  }
  tags
}

pub fn generate_many_tag_mocks(from: u32, to: u32) -> Vec<Tag> {
  generate_many_tag_mocks_with_overwrite(
    from,
    to,
    Some(|_id| PartialTag {
      id: None,
      slug: None,
      name: None,
    }),
  )
}

pub fn generate_tags_seed() -> Vec<Tag> {
  let roles = [
    "developer",
    "engineer",
    "designer",
    "manager",
    "tester",
    "devops",
    "qa",
  ];
  let programming_languages = [
    "rust",
    "python",
    "javascript",
    "go",
    "typescript",
    "c++",
    "c",
    "java",
  ];
  let skills = [
    "backend",
    "frontend",
    "fullstack",
    "devops",
    "qa",
    "ux",
    "ui",
  ];
  let experience_levels = [
    "junior",
    "mid",
    "senior",
    "team lead",
    "architect",
    "engineering lead",
  ];

  let medical_tags = [
    "doctor",
    "nurse",
    "pharmacist",
    "dentist",
    "surgeon",
    "therapist",
    "optometrist",
    "psychologist",
    "veterinarian",
    "dietitian",
    "nutritionist",
    "chiropractor",
    "pediatrician",
    "treat",
    "patients",
  ];

  let all_tag_names = [
    roles.as_ref(),
    programming_languages.as_ref(),
    skills.as_ref(),
    experience_levels.as_ref(),
    medical_tags.as_ref(),
  ]
  .concat();

  let total_tags_len = all_tag_names.len();
  generate_many_tag_mocks_with_overwrite(
    0,
    total_tags_len as u32,
    Some(|id| {
      let tag_name = all_tag_names[id as usize];
      PartialTag {
        id: None,
        slug: Some(tag_name.to_string().replace("+", "_plus").replace(" ", "_")),
        name: Some(titlecase(tag_name)),
      }
    }),
  )
}
