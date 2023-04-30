import { createSlice } from "@reduxjs/toolkit";
import { PostCardProps } from "src/components/card/post";
import { LOADABLE } from "src/utils/loadable";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface LandingPageState {
  posts: LOADABLE<Array<PostCardProps["post"]>>;
}

export const landingPage = createSlice({
  name: "landingPage",
  initialState: { posts: null } as LandingPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
