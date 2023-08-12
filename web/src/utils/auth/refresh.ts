import { fetch } from "../fetch/fetch";
import { authSave } from "./save";

export const authRefresh = async () => {
  try {
    const {
      data: { auth_token },
    } = await fetch.post<{ auth_token: string }>("/auth/refresh_token");

    authSave(auth_token);
  } catch (error) {
    console.log("Error refreshing auth token");
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
    // @TODO-ZM: use Logger abstraction instead of console
  }
};
