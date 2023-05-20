import { createSlice } from "@reduxjs/toolkit";
import { PostCardProps } from "src/components/card/post";
import { LOADABLE } from "src/utils/loadable";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface LandingPageState {
  total_post_count: number;
  posts: LOADABLE<Array<PostCardProps["post"]>>;
  query: string;
}

export const landingPage = createSlice({
  name: "landingPage",
  initialState: { posts: null, query: "", total_post_count: 0 } as LandingPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
