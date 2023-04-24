import { FC } from "react";
import { Input } from "src/components/input";
import { Stack } from "src/components/stack";
import { StyleProps } from "src/utils/props/style";
import { JobPost } from "src/pages/landing/state";
import { Text } from "src/components/text";

export interface JobPostCardProps extends StyleProps {
  job_post: JobPost;
}

export const JobPostCard: FC<JobPostCardProps> = ({ margin, job_post }) => {
  return (
    <Stack orientation="vertical" margin={margin}>
      <Text variant="v4">{job_post.title}</Text>
      <Text variant="v5">{job_post.short_description}</Text>
    </Stack>
  );
};
