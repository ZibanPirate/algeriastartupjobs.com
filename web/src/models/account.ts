export type AccountType =
  | {
      type: "Admin";
      first_name: string;
      last_name: string;
    }
  | {
      type: "Individual";
      first_name: string;
      last_name: string;
    }
  | {
      type: "Company";
      company_name: string;
    };

export type Account = {
  id: number;
  slug: string;
  email: string;
} & AccountType;

export type CompactAccount = Pick<Account, "id" | "slug" | "type"> & AccountType;
