import memoize from "lodash/memoize";
import { getStage } from "./get-stage";

const { stageIndex } = getStage();

const _getConfig = () => ({
  api: {
    base_url: [
      `http://${location.hostname}:9090`,
      "https://staging.api.magiframe.com",
      "https://production.api.magiframe.com",
    ][stageIndex],
  },
  web: {
    base_url: [
      `http://${location.hostname}:3000`,
      "https://staging.magiframe.com",
      "https://www.magiframe.com",
    ][stageIndex],
    version: APP_VERSION,
  },
});
export const getConfig = memoize(_getConfig);
