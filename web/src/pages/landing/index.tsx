import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchJobPostsForLanding } from "./actions";

export default (): ReturnType<FC> => {
  const { jobPosts } = useSliceSelector("landingPage");

  useEffect(() => {
    fetchJobPostsForLanding();
  }, []);

  return (
    <Stack orientation="vertical">
      <div className="absolute-center">
        <Stack orientation="vertical">
          <Text variant="v1" margin="1">
            Join a startup in Algeria
          </Text>
          <Text variant="v4" margin="1">
            Source code is publicly available at{" "}
            <Link
              to="https://github.com/algeriastartupjobs/algeriastartupjobs.com"
              variant="v4"
            >
              Github
            </Link>
          </Text>
          <Text variant="v4" margin="1">
            {JSON.stringify(jobPosts, null, 2)}
          </Text>
        </Stack>
      </div>
    </Stack>
  );
};
