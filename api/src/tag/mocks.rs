use super::model::{PartialTag, Tag};
use titlecase::titlecase;

pub fn generate_one_tag_mock(tag_id: i32) -> Tag {
    Tag {
        id: tag_id,
        slug: format!("tag_{}", tag_id),
        name: format!("Tag {}", tag_id),
    }
}

pub fn generate_many_tag_mocks_with_overwrite<F>(
    from: i32,
    to: i32,
    overwrite: Option<F>,
) -> Vec<Tag>
where
    F: Fn(i32) -> PartialTag,
{
    let mut tags: Vec<Tag> = Vec::new();
    for i in from..to {
        let tag = match overwrite {
            Some(ref f) => {
                let partial_tag = f(i);
                let default_tag = generate_one_tag_mock(i);
                // @TODO-ZM: write a marco optional_override!
                Tag {
                    id: partial_tag.id.unwrap_or(default_tag.id),
                    slug: partial_tag.slug.unwrap_or(default_tag.slug),
                    name: partial_tag.name.unwrap_or(default_tag.name),
                }
            }
            None => generate_one_tag_mock(i),
        };
        tags.push(tag);
    }
    tags
}

pub fn generate_many_tag_mocks(from: i32, to: i32) -> Vec<Tag> {
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
    let all_tag_names = [
        roles.as_ref(),
        programming_languages.as_ref(),
        skills.as_ref(),
        experience_levels.as_ref(),
    ]
    .concat();

    let total_tags_len = all_tag_names.len();
    generate_many_tag_mocks_with_overwrite(
        0,
        total_tags_len as i32,
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
