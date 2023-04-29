use super::model::{Account, AccountType, PartialAccount};
use titlecase::titlecase;

pub fn generate_one_account_mock(account_id: i32) -> Account {
    Account {
        id: account_id,
        slug: format!("account_{}", account_id),
        r#type: AccountType::Individual {
            first_name: format!("Moh {}", account_id),
            last_name: format!("Debbabi {}", account_id),
        },
        email: format!("moh_debbabi_mock_{}@algeriastartupjobs.com", account_id),
    }
}

pub fn generate_many_account_mocks_with_overwrite<F>(
    from: i32,
    to: i32,
    overwrite: Option<F>,
) -> Vec<Account>
where
    F: Fn(i32) -> PartialAccount,
{
    let mut accounts: Vec<Account> = Vec::new();
    for i in from..to {
        let account = match overwrite {
            Some(ref f) => {
                let partial_account = f(i);
                let default_account = generate_one_account_mock(i);
                // @TODO-ZM: write a marco optional_override!
                Account {
                    id: partial_account.id.unwrap_or(default_account.id),
                    slug: partial_account.slug.unwrap_or(default_account.slug),
                    r#type: partial_account.r#type.unwrap_or(default_account.r#type),
                    email: partial_account.email.unwrap_or(default_account.email),
                }
            }
            None => generate_one_account_mock(i),
        };
        accounts.push(account);
    }
    accounts
}

pub fn generate_many_account_mocks(from: i32, to: i32) -> Vec<Account> {
    generate_many_account_mocks_with_overwrite(
        from,
        to,
        Some(|_id| PartialAccount {
            id: None,
            slug: None,
            r#type: None,
            email: None,
        }),
    )
}

pub fn generate_accounts_seed() -> Vec<Account> {
    let admins = ["Zakaria Mansouri"];

    let algerian_startups = [
        "Yassiron",
        "Specific Emballage",
        "Startdown.dz",
        "Algeria Startdown Test",
        "Startupstare",
    ];
    let algerian_self_entrepreneurs = [
        "Issab Refraf",
        "Alia Haddada",
        "Fatima Zeroual",
        "Mourat Weld El Ailma",
        "Nadiatora Ramdani",
    ];

    let all_account_names = [
        admins.as_ref(),
        algerian_startups.as_ref(),
        algerian_self_entrepreneurs.as_ref(),
    ]
    .concat();

    let total_accounts_len = all_account_names.len();

    generate_many_account_mocks_with_overwrite(
        0,
        total_accounts_len as i32,
        Some(|id| {
            let account_name = all_account_names[id as usize];
            let mut iter = account_name.split_whitespace();
            let first_name = iter.next().unwrap();
            let last_name = iter.collect::<Vec<_>>().join(" ");
            PartialAccount {
                id: None,
                slug: Some(format!(
                    "{}_{}",
                    account_name.to_string().to_lowercase().replace(" ", "_"),
                    id
                )),
                r#type: match admins.contains(&account_name) {
                    true => Some(AccountType::Admin {
                        first_name: titlecase(first_name),
                        last_name: titlecase(last_name.as_str()),
                    }),
                    false => match algerian_self_entrepreneurs.contains(&account_name) {
                        true => Some(AccountType::Individual {
                            first_name: titlecase(first_name),
                            last_name: titlecase(last_name.as_str()),
                        }),
                        false => Some(AccountType::Company {
                            company_name: titlecase(account_name),
                        }),
                    },
                },
                email: Some(
                    format!(
                        "jobs_at_{}_mock_{}@algeriastartupjobs.com",
                        account_name, id,
                    )
                    .to_lowercase()
                    .replace(" ", "_"),
                ),
            }
        }),
    )
}
