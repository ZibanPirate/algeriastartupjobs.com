import { createSlice } from "@reduxjs/toolkit";
import { Account } from "src/models/account";
import { LOADABLE } from "src/utils/loadable";
import { setterReducerFactory } from "src/utils/state/reducer";

export interface ConfirmEmailPageState {
  confirmation_id: string;
  confirmation_code: string;
  post_id: number;
  confirmation_status: "IDLE" | "CONFIRMING" | "CONFIRMED" | "ERROR";
}

export const initialStateForConfirmEmailPage: ConfirmEmailPageState = {
  confirmation_id: "",
  confirmation_code: "",
  post_id: 0,
  confirmation_status: "IDLE",
};

export const confirmEmailPage = createSlice({
  name: "confirmEmailPage",
  initialState: initialStateForConfirmEmailPage,
  reducers: {
    set: setterReducerFactory(),
  },
});
