import { createSlice } from "@reduxjs/toolkit";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export interface ImportPageState {
  url: string;
}

export const initialStateForImportPage: ImportPageState = {
  url: "",
};

export const importPage = createSlice({
  name: "importPage",
  initialState: initialStateForImportPage,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
