import { LOADABLE } from "src/types/state";
import { createSlice } from "@reduxjs/toolkit";
import { setterReducerFactory } from "src/utils/state/reducer";

interface JobPost {
  id: string;
  title: string;
  description: string;
}

export interface LandingPageState {
  jobPosts: LOADABLE<JobPost[]>;
}

export const landingPage = createSlice({
  name: "landingPage",
  initialState: {
    jobPosts: null,
  } as LandingPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
