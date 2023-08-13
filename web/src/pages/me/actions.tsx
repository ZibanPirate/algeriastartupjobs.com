import { Account } from "src/models/account";
import { getState, getStateActions } from "src/state";
import { fetch } from "src/utils/fetch/fetch";
import * as Sentry from "@sentry/react";

// @TODO-ZM: throttle this and other actions as well
export const fetchAccountForMePage = async () => {
  const { mePage, accountEntities } = getStateActions();
  const { account } = getState().mePage;
  if (account === "ERROR") mePage.set({ account: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await fetch.get<{
      account: Account;
    }>("/accounts/me");

    mePage.set({ account: data.account });

    // update cache:
    accountEntities.upsertMany([data.account]);
  } catch (error) {
    mePage.set({ account: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching post for post page", error);
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
