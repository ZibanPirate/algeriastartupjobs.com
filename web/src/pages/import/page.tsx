import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";
import { Icon } from "src/components/icon";
import { CREATE_POST_PAGE_URL } from "src/utils/urls/common";
import { Input } from "src/components/input";
import { useSliceSelector } from "src/utils/state/selector";
import { getStateActions } from "src/state";
import { Button } from "src/components/button";
import { importFromURL } from "./actions";

export const Page: FC = () => {
  usePageTitle("Import your job post from other platforms");

  const { url } = useSliceSelector("importPage");
  const { set } = getStateActions().importPage;

  return (
    <Stack orientation="vertical" fullWidth align="center" maxWidth={600} margin="auto">
      <Stack orientation="vertical" stretch={true} align="start" padding="1 1 0">
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
          placeholder="URL from LinkedIn, Facebook etc."
          vtName="global-search"
          stretch={true}
        />
        <Stack orientation="vertical" align="center" stretch>
          <Stack orientation="horizontal" align="center" gap="1">
            <Button variant="v3" vtName="new-post" onClick={importFromURL}>
              Import now
            </Button>
          </Stack>
        </Stack>
      </Stack>
    </Stack>
  );
};
