import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchJobsForFirstCategoryForLanding } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";
import { GlobalSearch } from "src/components/search/global";

import { JobPostCard } from "src/components/card/job-post";
import { Button } from "src/components/button";

export const Page: FC = () => {
  usePageTitle("Join a startup in Algeria");

  const { jobsPostsGroupedByCategory } = useSliceSelector("landingPage");

  useEffect(() => {
    fetchJobsForFirstCategoryForLanding();
  }, []);

  return (
    <Stack orientation="vertical">
      {/* Header */}
      <Stack
        orientation="vertical"
        margin="3 0 0"
        stretch={true}
        align="center"
      >
        <Text variant="v1" margin="0 1">
          Join a startup in Algeria
        </Text>
        <Text variant="v4" margin="1 1">
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
      <Stack
        orientation="vertical"
        margin="1 0 2"
        stretch={true}
        align="center"
      >
        <GlobalSearch margin="0 1" />
      </Stack>
      {/* Jobs */}
      <Stack orientation="horizontal" margin="0 1">
        {jobsPostsGroupedByCategory.map((item) => (
          <Stack orientation="vertical" key={item.category.name}>
            <Text variant="v3" margin="0 0 1">
              {item.category.name} Jobs
            </Text>
            <Stack orientation="vertical" margin="0 0 1" gap="1">
              {item.job_posts === "ERROR" ? (
                <Stack orientation="horizontal" align="baseline">
                  <Text variant="v5" margin="0 0 1">
                    An error occured while fetching jobs, please &nbsp;
                  </Text>
                  <Button
                    variant="v5"
                    onClick={fetchJobsForFirstCategoryForLanding}
                  >
                    Try Again
                  </Button>
                </Stack>
              ) : item.job_posts ? (
                item.job_posts.map((jobPost) => (
                  <JobPostCard job_post={jobPost} />
                ))
              ) : (
                "@TODO-ZM: Loading..."
              )}
            </Stack>
            <Link
              to={`/jobs/${item.category.name}`}
              variant="v5"
              margin="0 0 1"
            >
              See all {item.category.name} jobs
            </Link>
          </Stack>
        ))}
      </Stack>
      {/* Footer */}
    </Stack>
  );
};
