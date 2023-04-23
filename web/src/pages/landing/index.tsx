import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchMilestonesForLanding } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";
import { GlobalSearch } from "src/components/search/global";

export const Page: FC = () => {
  usePageTitle("Join a startup in Algeria");

  const { milestones } = useSliceSelector("landingPage");

  useEffect(() => {
    fetchMilestonesForLanding();
  }, []);

  return (
    <Stack orientation="vertical" align="center">
      {/* Header */}
      <Stack orientation="vertical" margin="3 1">
        <Text variant="v1" margin="0 0 1">
          Join a startup in Algeria
        </Text>
        <Text variant="v4" margin="0 0 1">
          Source code is publicly available at&nbsp;
          <Link
            to="https://github.com/algeriastartupjobs/algeriastartupjobs.com"
            variant="v4"
          >
            Github
          </Link>
        </Text>
      </Stack>
      {/* Global Search */}
      <Stack orientation="vertical" margin="3 1">
        <GlobalSearch margin="1" />
      </Stack>
      {/* Jobs */}
      {/* Milestones */}
      <Text variant="v4">
        <pre>{JSON.stringify({ milestones }, null, 2)}</pre>
      </Text>
      {/* Footer */}
    </Stack>
  );
};
