import { createSlice } from "@reduxjs/toolkit";
import { Account } from "src/models/account";
import { LOADABLE } from "src/utils/loadable";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export interface MePageState {
  account: LOADABLE<Account>;
}

export const initialStateForMePage: MePageState = {
  account: null,
};

export const mePage = createSlice({
  name: "mePage",
  initialState: initialStateForMePage,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
