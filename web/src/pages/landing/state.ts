import { LOADABLE } from "src/types/state";
import { createSlice } from "@reduxjs/toolkit";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface MileStone {
  id: string;
  title: string;
  description: string;
  deadline: Date;
  progress: number;
  completed: boolean;
}

export interface LandingPageState {
  milestones: LOADABLE<MileStone[]>;
}

export const landingPage = createSlice({
  name: "landingPage",
  initialState: {
    milestones: null,
  } as LandingPageState,
  reducers: {
    set: setterReducerFactory(),
  },
});
