import { createSlice } from "@reduxjs/toolkit";
import { Account } from "src/models/account";
import { Category } from "src/models/category";
import { Tag } from "src/models/tag";
import { LOADABLE } from "src/utils/loadable";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export type CreatePostPageState =
  | {
      title: string;
      poster_type: Account["type"];
      poster_name: string;
      poster_first_name: string;
      poster_last_name: string;
      poster_contact: string;
      poster: LOADABLE<Account>;
      creation_status: "IDLE" | "CREATING" | "CREATED" | "ERROR";
      suggested_tags: LOADABLE<Tag[]>;
      suggested_categories: LOADABLE<Category[]>;
    } & (
      | {
          compact: true;
          post_description?: never;
          tags?: never;
          category?: never;
        }
      | {
          compact: false;
          post_description: string;
          tags: Tag[];
          category?: Category;
        }
    );

export const initialStateForCreatePostPage: CreatePostPageState = {
  title: "",
  poster_type: "Company",
  poster_name: "",
  poster_first_name: "",
  poster_last_name: "",
  poster_contact: "",
  poster: null,
  creation_status: "IDLE",
  compact: true,
  suggested_tags: null,
  suggested_categories: null,
};

export const createPostPage = createSlice({
  name: "createPostPage",
  initialState: initialStateForCreatePostPage as CreatePostPageState,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
