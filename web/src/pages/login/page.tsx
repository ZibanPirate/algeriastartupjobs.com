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
import { Footer } from "src/components/footer";

export const Page: FC = () => {
  usePageTitle("Login to the system to manage your data");

  const { email, login_status } = useSliceSelector("loginPage");
  const { set } = getStateActions().loginPage;

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
            {["LOGGING_IN", "CODE_SENT"].includes(login_status) ? (
              <Icon
                variant="v3"
                name={login_status === "CODE_SENT" ? "success" : "loadingSpinner"}
                animation={login_status !== "CODE_SENT" ? "rotate" : undefined}
                margin="3 0"
              />
            ) : (
              <>
                <Text variant="v4" margin="1 0 2">
                  {login_status === "ERROR" ? "Something went wrong, please try again" : <br />}
                </Text>
                <Stack orientation="horizontal" align="center" gap="1">
                  <Button variant="v3" vtName="login-button" onClick={() => login()}>
                    Login
                  </Button>
                </Stack>
              </>
            )}
          </Stack>
        </Stack>
      </Stack>
      <Footer />
    </Stack>
  );
};
