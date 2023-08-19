import { getStateActions } from "src/state";
import * as Sentry from "@sentry/react";
import { ImportStatus, initialStateForImportStatusPage } from "./state";
import { createFetchStream } from "src/utils/fetch/create-fetch-stream";
import { onceAtATime } from "src/utils/concurrency/once-at-a-time";
import { getBrowserRouter } from "src/components/router-provider";
import { DRAFT_PAGE_URL } from "src/utils/urls/common";
import { viewTransitionSubscribeOnce } from "src/utils/animation/view-transition";
import { ANIMATION_DURATION } from "src/utils/animation/duration";
import { initialStateForImportPage } from "../import/state";

type ImportStatusResponse =
  | { status: Exclude<ImportStatus, "DONE">; draft_id?: never }
  | { status: Extract<ImportStatus, "DONE">; draft_id: number };

const _fetchImportStatusForURL = async (url: string): Promise<void> => {
  const { importStatusPage, importPage } = getStateActions();
  const initialLocation = getBrowserRouter().state.location;
  try {
    const { listen, close } = createFetchStream<ImportStatusResponse>({
      url: `/import/status?url=${url}`,
    });

    let response: ImportStatusResponse | null = null;
    // @TODO-ZM: handle network errors
    while ((response = await listen())) {
      if (initialLocation !== getBrowserRouter().state.location) {
        importStatusPage.overwrite(initialStateForImportStatusPage);
        close();
        return;
      }
      importStatusPage.set({ status: response.status });
      if (response.status === "DONE") {
        const draft_id = response.draft_id;
        setTimeout(() => {
          viewTransitionSubscribeOnce(() => {
            importStatusPage.overwrite(initialStateForImportStatusPage);
            importPage.overwrite(initialStateForImportPage);
          });
          getBrowserRouter().navigate(`${DRAFT_PAGE_URL}/${draft_id}`);
        }, ANIMATION_DURATION);
      }
    }
  } catch (error) {
    importStatusPage.set({ status: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching post count for landing page", error);
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};

export const fetchImportStatusForURL = onceAtATime(_fetchImportStatusForURL);
