import { PayloadAction } from "@reduxjs/toolkit";

export const setterReducer = <S>(
  state: S,
  action: PayloadAction<Partial<S>>
) => {
  (Object.keys(action.payload) as (keyof typeof action.payload)[]).forEach(
    (key) => {
      if (typeof action.payload[key] !== "undefined") {
        state[key] = action.payload[key] as S[keyof S];
      }
    }
  );
};

export const setterReducerFactory =
  <S>() =>
  (state: S, action: PayloadAction<Partial<S>>) =>
    setterReducer(state, action);
