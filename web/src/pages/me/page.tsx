import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { POST_PAGE_URL } from "src/utils/urls/common";
import { Button } from "src/components/button";
import { useIsAuthenticated } from "src/utils/hooks/is-authenticated";
import { useSliceSelector } from "src/utils/state/selector";
import { authRevoke } from "src/utils/auth/revoke";
import { fetchAccountForMePage } from "./actions";
import { Footer } from "src/components/footer";

export const Page: FC = () => {
  usePageTitle("My Account");

  const { isAuthenticated } = useIsAuthenticated();
  const { account } = useSliceSelector("mePage");

  useEffect(() => {
    if (isAuthenticated) fetchAccountForMePage();
  }, [isAuthenticated]);

  return (
    <Stack
      orientation="vertical"
      fullWidth
      align="center"
      minHeight="100vh"
      justifyContent="space-between"
    >
      <Stack orientation="vertical" fullWidth align="center" maxWidth={600} margin="auto">
        <Stack
          orientation="horizontal"
          stretch={true}
          justifyContent="space-between"
          align="center"
          padding="1 1 0"
        >
          <Link variant="v4" back={POST_PAGE_URL} to={"/"} vtName="login">
            <Icon variant="v4" name="back" /> Back
          </Link>
          {isAuthenticated && (
            <Button variant="v4" onClick={authRevoke}>
              Logout
            </Button>
          )}
        </Stack>
        {!isAuthenticated ? (
          <Stack orientation="vertical" align="center" stretch gap="1" padding="3 1">
            <Icon variant="v1" name="login" vtName="login-icon" />
            <Text variant="v3">Please login to see your account details</Text>
            <Link variant="v4" to={"/login"}>
              <Button variant="v4" vtName="login-button" margin="1 0 2">
                Login now
              </Button>
            </Link>
          </Stack>
        ) : (
          <Stack orientation="vertical" align="center" stretch gap="1" padding="3 1">
            {/* @TODO-ZM: proper load-friendly UI */}
            <pre>{JSON.stringify(account, null, 2)}</pre>
          </Stack>
        )}
      </Stack>
      <Footer />
    </Stack>
  );
};
