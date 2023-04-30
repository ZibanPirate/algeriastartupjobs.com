export type AccountType =
  | {
      type: "admin";
      first_name: string;
      last_name: string;
    }
  | {
      type: "individual";
      first_name: string;
      last_name: string;
    }
  | {
      type: "company";
      company: string;
    };

export type Account = {
  id: number;
  slug: string;
  email: string;
} & AccountType;

export type CompactAccount = Pick<Account, "id" | "slug" | "type"> &
  AccountType;
