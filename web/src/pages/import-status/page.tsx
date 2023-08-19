import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { IMPORT_PAGE_URL } from "src/utils/urls/common";
import { useSliceSelector } from "src/utils/state/selector";
import { useSearchParams } from "react-router-dom";
import { fetchImportStatusForURL } from "./actions";

export const Page: FC = () => {
  const { status } = useSliceSelector("importStatusPage");

  const [searchParams] = useSearchParams();
  const url = searchParams.get("url");

  useEffect(() => {
    if (url) fetchImportStatusForURL(url);
  }, []);

  const statusText =
    status === "FETCHING"
      ? "Fetching ..."
      : status === "PROCESSING"
      ? "Processing ..."
      : status === "DONE"
      ? "Imported successfully"
      : status === "ERROR"
      ? "Something went wrong"
      : "";

  usePageTitle(statusText);

  return (
    <Stack orientation="vertical" fullWidth align="center" maxWidth={600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="start">
        <Link variant="v4" back={IMPORT_PAGE_URL} to={"/"} vtName="login">
          <Icon variant="v4" name="back" /> Back
        </Link>
      </Stack>
      <Stack orientation="vertical" align="center" stretch gap="1" padding="3 1">
        <Text variant="v3">{statusText}</Text>
        <Icon
          variant="v1"
          // @TODO-ZM: use proper error icon
          name={status === "DONE" ? "success" : status === "ERROR" ? "removeTag" : "loadingSpinner"}
          vtName="login-icon"
          animation={["DONE", "ERROR"].includes(status) ? "none" : "rotate"}
        />
      </Stack>
      <Text variant="v4" margin="0 1 1">
        Source code is publicly available at&nbsp;
        <Link to="https://github.com/algeriastartupjobs/algeriastartupjobs.com" variant="v4">
          Github
        </Link>
      </Text>
    </Stack>
  );
};
