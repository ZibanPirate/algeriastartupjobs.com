import { createEntityAdapter, createSlice } from "@reduxjs/toolkit";
import { CompactCategory, Category } from "src/models/category";

export type CategoryEntity = CompactCategory & Partial<Category>;

const categoryEntitiesAdapter = createEntityAdapter<CategoryEntity>({
  selectId: (category) => category.id,
  sortComparer: (a, b) => b.id - a.id,
});

export const categoryEntities = createSlice({
  name: "categoryEntities",
  initialState: categoryEntitiesAdapter.getInitialState(),
  reducers: {
    upsertMany: categoryEntitiesAdapter.upsertMany,
  },
});
