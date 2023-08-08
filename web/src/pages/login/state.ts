import { createSlice } from "@reduxjs/toolkit";
import { overWriterReducerFactory, setterReducerFactory } from "src/utils/state/reducer";

export interface LoginPageState {
  confirmation_id: string;
  confirmation_code: string;
  email: string;
  login_status: "IDLE" | "LOGGING_IN" | "CODE_SENT" | "CONFIRMING" | "CONFIRMED" | "ERROR";
}

export const initialStateForLoginPage: LoginPageState = {
  confirmation_id: "",
  confirmation_code: "",
  email: "",
  login_status: "IDLE",
};

export const loginPage = createSlice({
  name: "loginPage",
  initialState: initialStateForLoginPage,
  reducers: {
    set: setterReducerFactory(),
    overwrite: overWriterReducerFactory(),
  },
});
