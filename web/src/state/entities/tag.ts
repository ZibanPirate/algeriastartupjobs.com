import { createEntityAdapter, createSlice } from "@reduxjs/toolkit";
import { CompactTag, Tag } from "src/models/tag";

export type TagEntity = CompactTag & Partial<Tag>;

const tagEntitiesAdapter = createEntityAdapter<TagEntity>({
  selectId: (tag) => tag.id,
  sortComparer: (a, b) => b.id - a.id,
});

export const tagEntities = createSlice({
  name: "tagEntities",
  initialState: tagEntitiesAdapter.getInitialState(),
  reducers: {
    upsertMany: tagEntitiesAdapter.upsertMany,
  },
});
