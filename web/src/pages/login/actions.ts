import { getState, getStateActions } from "src/state";
import { initialStateForLoginPage } from "./state";
import { getBrowserRouter } from "src/components/router-provider";
import { CONFIRM_LOGIN_PAGE_URL } from "src/utils/urls/common";
import { initialStateForConfirmLoginPage } from "../confirm-login/state";
import { fetch } from "src/utils/fetch/fetch";
import { viewTransitionSubscribeOnce } from "src/utils/animation/view-transition";
import * as Sentry from "@sentry/react";

export const login = async () => {
  const { loginPage, confirmLoginPage } = getStateActions();
  loginPage.set({ login_status: "LOGGING_IN" });

  try {
    const { email } = getState().loginPage;

    const {
      data: { confirmation_id },
    } = await fetch.post<{ confirmation_id: string }>("/auth/login", {
      email,
    });

    // @TODO-ZM: fix typing on overwrite reducer.
    loginPage.set({ login_status: "CODE_SENT" });
    viewTransitionSubscribeOnce(() => {
      loginPage.overwrite(initialStateForLoginPage);
    });
    confirmLoginPage.set({ ...initialStateForConfirmLoginPage, confirmation_id, email });
    getBrowserRouter().navigate(CONFIRM_LOGIN_PAGE_URL);
  } catch (error) {
    console.log("Error creating post", error);
    // @TODO-ZM: use Logger abstraction instead of console
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
    loginPage.set({ login_status: "ERROR" });
  }
};
