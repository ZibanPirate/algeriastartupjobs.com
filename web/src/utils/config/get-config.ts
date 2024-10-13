import memoize from "lodash/memoize";
import { getStage } from "./get-stage";

const { stageIndex } = getStage();

const _getConfig = () => ({
  api: {
    base_url: [
      `http://${location.hostname}:9090`,
      "https://staging.api.dzjob.io",
      "https://production.api.dzjob.io",
    ][stageIndex],
  },
  web: {
    base_url: [
      `http://${location.hostname}:3000`,
      "https://staging.dzjob.io",
      "https://www.dzjob.io",
    ][stageIndex],
    version: APP_VERSION,
  },
});
export const getConfig = memoize(_getConfig);
