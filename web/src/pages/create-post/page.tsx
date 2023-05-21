import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { usePageTitle } from "src/utils/hooks/page-title";

import { useNavigate } from "react-router-dom";
import { getStateActions } from "src/state";
import { GlobalSearch } from "src/components/search/global";
import { Icon } from "src/components/icon";
import { fetchPostCountForLandingPage, fetchPostsForLandingPage } from "src/pages/landing/actions";

export const Page: FC = () => {
  const { query, total_post_count } = useSliceSelector("landingPage");
  const navigate = useNavigate();

  useEffect(() => {
    fetchPostsForLandingPage();
  }, [query]);

  useEffect(() => {
    fetchPostCountForLandingPage();
  }, []);

  usePageTitle("Post a job add for free!");

  return (
    <Stack orientation="vertical" stretch align="center" maxWidth={1600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" stretch={true} align="stretch">
        <Stack orientation="horizontal" margin="1 1 0" gap="1" align="space-between">
          <Stack orientation="vertical" align="start">
            <Link variant="v4" back={true} to={"/"}>
              <Icon variant="v4" name="back" /> Back
            </Link>
          </Stack>
          <Stack orientation="vertical" align="center">
            <GlobalSearch
              total_post_count={total_post_count}
              value={query}
              setValue={(value) => getStateActions().landingPage.set({ query: value })}
              onClick={() => navigate("/")}
            />
          </Stack>
        </Stack>
      </Stack>
      {/* Post */}
      <Stack orientation="horizontal" align="start" stretch={true} gap="3">
        Post form goes here
      </Stack>
      <Text variant="v4" margin="2 1">
        Source code is publicly available at&nbsp;
        <Link to="https://github.com/algeriastartupjobs/algeriastartupjobs.com" variant="v4">
          Github
        </Link>
      </Text>
    </Stack>
  );
};
