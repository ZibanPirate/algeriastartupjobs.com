import { AccountType } from "src/models/account";

export const getAccountName = (account: AccountType) => {
  switch (account.type) {
    case "Company":
      return account.company_name;
    case "Admin":
    case "Individual":
      return `${account.first_name} ${account.last_name}`;
  }
};
