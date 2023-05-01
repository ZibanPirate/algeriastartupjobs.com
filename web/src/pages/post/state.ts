import { createSlice } from "@reduxjs/toolkit";
import { Post } from "src/models/post";
import { LOADABLE } from "src/utils/loadable";
import { LoneModel } from "src/utils/models/lone-model";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface PostPageState {
  post: LOADABLE<LoneModel<Post>>;
}

export const postPage = createSlice({
  name: "postPage",
  initialState: { post: null } as PostPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
