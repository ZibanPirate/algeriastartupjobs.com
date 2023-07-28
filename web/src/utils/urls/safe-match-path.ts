import { matchPath } from "react-router-dom";

export const safeMatchPath: typeof matchPath = (...args) => {
  try {
    return matchPath(...args);
  } catch (e) {
    return null;
  }
};
