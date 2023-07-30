import { PayloadAction } from "@reduxjs/toolkit";

const setterReducer = <S extends Record<string, unknown>>(
  state: S,
  action: PayloadAction<Partial<S>>
) => {
  (Object.keys(action.payload) as (keyof typeof action.payload)[]).forEach((key) => {
    state[key] = action.payload[key] as S[keyof S];
  });
};

export const setterReducerFactory =
  <S extends Record<string, unknown>>() =>
  (state: S, action: PayloadAction<Partial<S>>) =>
    setterReducer(state, action);

const overWriterReducer = <S extends Record<string, unknown>>(
  state: S,
  action: PayloadAction<S>
) => {
  (Object.keys(state) as (keyof typeof state)[]).forEach((key) => delete state[key]);
  (Object.keys(action.payload) as (keyof typeof action.payload)[]).forEach((key) => {
    state[key] = action.payload[key] as S[keyof S];
  });
};

export const overWriterReducerFactory =
  <S extends Record<string, unknown>>() =>
  (state: S, action: PayloadAction<S>) =>
    overWriterReducer(state, action);
