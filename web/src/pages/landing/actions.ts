import { getStateActions } from "src/state";
import Axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { MileStone } from "./state";

export const fetchMilestonesForLanding = async (): Promise<void> => {
  const { landingPage } = getStateActions();
  try {
    landingPage.set({ milestones: null });
    // @TODO-ZM: use fetchV2
    // const { milestones } = await fetchV2("api:MileStones/dzcode", {});
    const {
      data: { milestones },
    } = await Axios.get<{ milestones: MileStone[] }>(
      getConfig().api.base_url + "/milestones"
    );

    landingPage.set({ milestones });
  } catch (error) {
    landingPage.set({ milestones: "ERROR" });
    // @TODO-ZM: add sentry
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
