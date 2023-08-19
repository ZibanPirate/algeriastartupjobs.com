import { getState } from "src/state";
import { getBrowserRouter } from "src/components/router-provider";
import { IMPORT_STATUS_PAGE_URL } from "src/utils/urls/common";

export const importFromURL = async (): Promise<void> => {
  const { url } = getState().importPage;

  getBrowserRouter().navigate(`${IMPORT_STATUS_PAGE_URL}?url=${url}`);
};
