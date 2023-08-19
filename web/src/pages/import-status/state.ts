import { createSlice } from "@reduxjs/toolkit";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export type ImportStatus = "FETCHING" | "PROCESSING" | "DONE" | "ERROR";

export interface ImportStatusPageState {
  status: ImportStatus;
}

export const initialStateForImportStatusPage: ImportStatusPageState = {
  status: "FETCHING",
};

export const importStatusPage = createSlice({
  name: "importStatusPage",
  initialState: initialStateForImportStatusPage,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
