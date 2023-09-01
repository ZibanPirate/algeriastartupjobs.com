import { createSlice } from "@reduxjs/toolkit";
import { PostCardProps } from "src/components/card/post";
import { CompactTag, Tag } from "src/models/tag";
import { LOADABLE } from "src/utils/loadable";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export interface PostsForPageState {
  posts: LOADABLE<Array<PostCardProps["post"]>>;
  tag: LOADABLE<CompactTag & Partial<Tag>>;
}

export const postsForPage = createSlice({
  name: "postsForPage",
  initialState: { posts: null, tag: null } as PostsForPageState,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
