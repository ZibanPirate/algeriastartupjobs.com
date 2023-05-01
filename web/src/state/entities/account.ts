import { createEntityAdapter, createSlice } from "@reduxjs/toolkit";
import { CompactAccount, Account } from "src/models/account";

export type AccountEntity = CompactAccount & Partial<Account>;

const accountEntitiesAdapter = createEntityAdapter<AccountEntity>({
  selectId: (account) => account.id,
  sortComparer: (a, b) => b.id - a.id,
});

export const accountEntities = createSlice({
  name: "accountEntities",
  initialState: accountEntitiesAdapter.getInitialState(),
  reducers: {
    upsertMany: accountEntitiesAdapter.upsertMany,
  },
});
