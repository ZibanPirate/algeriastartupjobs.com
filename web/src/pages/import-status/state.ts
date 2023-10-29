import { createSlice } from "@reduxjs/toolkit";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export type ImportStatus = "Pending" | "InProgress" | "Completed" | "Failed";

export interface ImportStatusPageState {
  status: ImportStatus;
}

export const initialStateForImportStatusPage: ImportStatusPageState = {
  status: "Pending",
};

export const importStatusPage = createSlice({
  name: "importStatusPage",
  initialState: initialStateForImportStatusPage,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
