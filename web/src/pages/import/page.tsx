import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { CREATE_POST_PAGE_URL, IMPORTED_PAGE_URL, POST_PAGE_URL } from "src/utils/urls/common";
import { Input } from "src/components/input";
import { useSliceSelector } from "src/utils/state/selector";
import { getStateActions } from "src/state";
import { Button } from "src/components/button";

export const Page: FC = () => {
  usePageTitle("Import your job post from other platforms");

  const { url } = useSliceSelector("importPage");
  const { set } = getStateActions().importPage;

  return (
    <Stack orientation="vertical" fullWidth align="center" maxWidth={600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="start">
        <Link variant="v4" back={CREATE_POST_PAGE_URL} to={"/"} vtName="login">
          <Icon variant="v4" name="back" /> Back
        </Link>
      </Stack>
      <Stack orientation="vertical" align="center" stretch gap="1" padding="3 1">
        <Icon variant="v1" name="import" vtName="login-icon" />
        <Text variant="v3">Paste URL</Text>
        <Input
          id="url"
          variant="v4"
          value={url}
          setValue={(value) => set({ url: value })}
          placeholder="URL"
          vtName="global-search"
          stretch={true}
        />
        <Stack orientation="vertical" align="center" stretch>
          <Stack orientation="horizontal" align="center" gap="1">
            <Link variant="v4" to={`${IMPORTED_PAGE_URL}?url=${url}`}>
              <Button variant="v3" vtName="new-post">
                Import now
              </Button>
            </Link>
          </Stack>
        </Stack>
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
