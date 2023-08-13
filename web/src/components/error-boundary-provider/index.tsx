import * as Sentry from "@sentry/react";
import { FC, PropsWithChildren } from "react";
import { Stack } from "src/components/stack";
import { Icon } from "src/components/icon";
import { Text } from "src/components/text";

export const ErrorBoundaryProvider: FC<PropsWithChildren> = ({ children }) => {
  return (
    <Sentry.ErrorBoundary
      fallback={
        <Stack orientation="horizontal">
          <Icon variant="v3" name="removeTag" />
          <Text variant="v3">Something went wrong</Text>
        </Stack>
      }
    >
      {children}
    </Sentry.ErrorBoundary>
  );
};
