import { createSlice } from "@reduxjs/toolkit";
import { AccountType, CompactAccount } from "src/models/account";
import { CompactCategory } from "src/models/category";
import { CompactPost, Post } from "src/models/post";
import { CompactTag } from "src/models/tag";
import { LOADABLE } from "src/utils/loadable";
import { LoneModel } from "src/utils/models/lone-model";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface PostPageState {
  post: LOADABLE<
    // @TODO-ZM: make CompactPost & Post work
    LoneModel<
      CompactPost &
        Partial<
          Post & {
            category: LoneModel<CompactCategory>;
            tags: Array<LoneModel<CompactTag>>;
            poster: LoneModel<CompactAccount> & AccountType;
          }
        >
    >
  >;
}

export const postPage = createSlice({
  name: "postPage",
  initialState: { post: null } as PostPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
