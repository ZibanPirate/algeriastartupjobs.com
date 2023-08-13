import { fetch } from "../fetch/fetch";
import { authSave } from "./save";
import * as Sentry from "@sentry/react";

export const authRefresh = async () => {
  try {
    const {
      data: { auth_token },
    } = await fetch.post<{ auth_token: string }>("/auth/refresh_token");

    authSave(auth_token);
  } catch (error) {
    // @TODO-ZM: use Logger abstraction instead of console
    console.log("Error refreshing auth token");
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
