import { combineReducers, compose, configureStore } from "@reduxjs/toolkit";
import * as slices from "./slices";
type Slices = typeof slices;
export type SliceName = keyof typeof slices;

type Reducers = { [K in keyof Slices]: Slices[K]["reducer"] };
const reducers = (Object.keys(slices) as SliceName[]).reduce(
  (pV, sliceKey) => ({ ...pV, [sliceKey]: slices[sliceKey].reducer }),
  {} as Reducers
);
const rootReducer = combineReducers(reducers);
export type State = ReturnType<typeof rootReducer>;

const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
const createStore = () =>
  configureStore({ reducer: rootReducer, enhancers: composeEnhancers });

let store: ReturnType<typeof createStore>;
type Actions = { [K in keyof Slices]: Slices[K]["actions"] };
let stateActions: Actions;

export const getStore = ({
  invalidateCache = false,
}: { invalidateCache?: boolean } = {}) => {
  if (invalidateCache || !store) {
    store = createStore();
    stateActions = (Object.keys(slices) as SliceName[]).reduce(
      (pV, sliceKey) => ({
        ...pV,
        [sliceKey]: Object.keys(slices[sliceKey].actions).reduce(
          (contextualActions, actionName) => ({
            ...contextualActions,
            [actionName]: (...args: any[]) =>
              store.dispatch(
                (slices[sliceKey].actions as any)[actionName](...args)
              ),
          }),
          {}
        ),
      }),
      {}
      // @TODO-ZM: stateActions type
    ) as unknown as Actions;
  }
  return store;
};

export const getStateActions = () => stateActions;
export const getState = () => getStore().getState();
