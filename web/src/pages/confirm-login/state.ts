import { createSlice } from "@reduxjs/toolkit";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export interface ConfirmLoginPageState {
  confirmation_id: string;
  confirmation_code: string;
  confirmation_status: "IDLE" | "CONFIRMING" | "CONFIRMED" | "ERROR";
}

export const initialStateForConfirmLoginPage: ConfirmLoginPageState = {
  confirmation_id: "",
  confirmation_code: "",
  confirmation_status: "IDLE",
};

export const confirmLoginPage = createSlice({
  name: "confirmLoginPage",
  initialState: initialStateForConfirmLoginPage,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
