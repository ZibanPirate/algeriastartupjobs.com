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
import { Footer } from "src/components/footer";

export const Page: FC = () => {
  const { status } = useSliceSelector("importStatusPage");

  const [searchParams] = useSearchParams();
  const url = searchParams.get("url");

  useEffect(() => {
    if (url) fetchImportStatusForURL(url);
  }, []);

  const statusText =
    status === "Pending"
      ? "Queued ..."
      : status === "InProgress"
      ? "Processing ..."
      : status === "Completed"
      ? "Imported successfully"
      : status === "Failed"
      ? "Something went wrong"
      : "";

  usePageTitle(statusText);

  return (
    <Stack
      orientation="vertical"
      fullWidth
      align="center"
      minHeight="100vh"
      justifyContent="space-between"
    >
      <Stack orientation="vertical" fullWidth align="center" maxWidth={600} margin="auto">
        <Stack orientation="vertical" stretch={true} align="start" padding="1 1 0">
          <Link variant="v4" back={IMPORT_PAGE_URL} to={"/"} vtName="login">
            <Icon variant="v4" name="back" /> Back
          </Link>
        </Stack>
        <Stack orientation="vertical" align="center" stretch gap="1" padding="3 1">
          <Text variant="v3">{statusText}</Text>
          <Icon
            variant="v1"
            // @TODO-ZM: use proper error icon
            name={
              status === "Completed"
                ? "success"
                : status === "Failed"
                ? "removeTag"
                : "loadingSpinner"
            }
            vtName="login-icon"
            animation={["DONE", "ERROR"].includes(status) ? "none" : "rotate"}
          />
        </Stack>
      </Stack>
      <Footer />
    </Stack>
  );
};
