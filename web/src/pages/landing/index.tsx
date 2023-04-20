import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchMilestonesForLanding } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";

export const Page: FC = () => {
  usePageTitle("Join a startup in Algeria");

  const { milestones } = useSliceSelector("landingPage");

  useEffect(() => {
    fetchMilestonesForLanding();
  }, []);

  return (
    <Stack orientation="vertical">
      <div className="absolute-center">
        <Stack orientation="vertical">
          <Text variant="v1" margin="1 0">
            Join a startup in Algeria
          </Text>
          <Text variant="v4" margin="0 0 1">
            Source code is publicly available at{" "}
            <Link
              to="https://github.com/algeriastartupjobs/algeriastartupjobs.com"
              variant="v4"
            >
              Github
            </Link>
          </Text>
          <Text variant="v4">
            <pre>{JSON.stringify({ milestones }, null, 2)}</pre>
          </Text>
        </Stack>
      </div>
    </Stack>
  );
};
