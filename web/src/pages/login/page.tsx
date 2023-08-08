import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { POST_PAGE_URL } from "src/utils/urls/common";
import { Input } from "src/components/input";
import { useSliceSelector } from "src/utils/state/selector";
import { getStateActions } from "src/state";
import { Button } from "src/components/button";
import { login } from "./actions";

export const Page: FC = () => {
  usePageTitle("Login to the system to manage your data");

  const { email, login_status } = useSliceSelector("loginPage");
  const { set } = getStateActions().loginPage;

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
        <Text variant="v3">Login via email</Text>
        <Input
          id="email"
          variant="v4"
          value={email}
          setValue={(value) => set({ email: value })}
          placeholder="Email"
          vtName="global-search"
          stretch={true}
        />
        <Stack orientation="vertical" align="center" stretch>
          {["LOGGING_IN", "CONFIRMING", "CONFIRMED"].includes(login_status) ? (
            <Icon
              variant="v3"
              name={login_status === "CONFIRMED" ? "success" : "loadingSpinner"}
              animation={login_status !== "CONFIRMED" ? "rotate" : undefined}
              margin="3 0"
              vtName="create-post-icon"
            />
          ) : (
            <>
              <Text variant="v4" margin="1 0 2">
                {login_status === "ERROR" ? "Something went wrong, please try again" : <br />}
              </Text>
              <Stack orientation="horizontal" align="center" gap="1">
                <Button variant="v3" onClick={() => login()}>
                  {login_status === "CODE_SENT" ? "Confirm code" : "Login"}
                </Button>
              </Stack>
            </>
          )}
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
