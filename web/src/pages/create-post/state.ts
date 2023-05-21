import { createSlice } from "@reduxjs/toolkit";
import { Account } from "src/models/account";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface CreatePostPageState {
  title: string;
  poster_type: Account["type"];
  poster_name: string;
  poster_contact: string;
}

export const createPostPage = createSlice({
  name: "createPostPage",
  initialState: {
    title: "",
    poster_type: "Company",
    poster_name: "",
    poster_contact: "",
  } as CreatePostPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
