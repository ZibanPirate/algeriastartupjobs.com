import { createEntityAdapter, createSlice } from "@reduxjs/toolkit";
import { CompactPost, Post } from "src/models/post";

export type PostEntity = CompactPost & Partial<Post>;

const postEntitiesAdapter = createEntityAdapter<PostEntity>({
  selectId: (post) => post.id,
  sortComparer: (a, b) => b.id - a.id,
});

export const postEntities = createSlice({
  name: "postEntities",
  initialState: postEntitiesAdapter.getInitialState(),
  reducers: {
    upsertMany: postEntitiesAdapter.upsertMany,
  },
});
