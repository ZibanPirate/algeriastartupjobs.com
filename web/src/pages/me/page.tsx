import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { POST_PAGE_URL } from "src/utils/urls/common";
import { Button } from "src/components/button";

export const Page: FC = () => {
  usePageTitle("My Account");

  return (
    <Stack orientation="vertical" fullWidth align="center" maxWidth={600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="start">
        <Link variant="v4" back={POST_PAGE_URL} to={"/"} vtName="login">
          <Icon variant="v4" name="back" /> Back
        </Link>
      </Stack>
      <Stack orientation="vertical" align="center" stretch gap="1" padding="3 1">
        <Icon variant="v1" name="login" vtName="login-icon" />
        <Text variant="v3">Please login to see your account details</Text>
        <Link variant="v4" to={"/login"}>
          <Button variant="v4" vtName="login-button" margin="1 0 2">
            Login now
          </Button>
        </Link>
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
