import { createSlice } from "@reduxjs/toolkit";
import { LOADABLE } from "src/utils/loadable";
import { setterReducerFactory } from "src/utils/state/reducer";

interface Account {
  name: string;
  avatar_url: string;
}

export interface JobPostCategory {
  name: string;
}

export interface JobPost {
  id: string;
  title: string;
  short_description: string;
  poster: Account;
}

export interface LandingPageState {
  jobsPostsGroupedByCategory: Array<{
    category: JobPostCategory;
    job_posts: LOADABLE<JobPost[]>;
  }>;
}

export const landingPage = createSlice({
  name: "landingPage",
  initialState: {
    milestones: null,
    jobsPostsGroupedByCategory: [{ category: { name: "IT" }, job_posts: null }],
  } as LandingPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
