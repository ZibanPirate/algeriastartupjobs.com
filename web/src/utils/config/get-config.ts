import memoize from "lodash/memoize";
import { getStage } from "./get-stage";

const { stageIndex } = getStage();

const _getConfig = () => ({
  api: {
    base_url: [
      `http://${location.hostname}:9090`,
      "https://staging.api.algeriastartupjobs.com",
      "https://production.api.algeriastartupjobs.com",
    ][stageIndex],
  },
  web: {
    base_url: [
      `http://${location.hostname}:3000`,
      "https://staging.algeriastartupjobs.com",
      "https://www.algeriastartupjobs.com",
    ][stageIndex],
    version: APP_VERSION,
  },
});
export const getConfig = memoize(_getConfig);
