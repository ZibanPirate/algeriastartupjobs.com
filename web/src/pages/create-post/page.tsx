import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { Input } from "src/components/input";
import { Button } from "src/components/button";

export const Page: FC = () => {
  usePageTitle("Post a job add for free!");

  return (
    <Stack orientation="vertical" stretch align="center" maxWidth={1600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="start">
        <Link variant="v4" back={true} to={"/"}>
          <Icon variant="v4" name="back" /> Back
        </Link>
      </Stack>
      {/* Create Post */}
      {/* @TODO-ZM: apply padding to other places where we did workaround it */}
      <Stack orientation="vertical" stretch gap="1" margin="3 0" padding="0 1">
        <Stack orientation="horizontal" gap="1" align="baseline">
          <Text variant="v4">Looking for</Text>
          <Input
            placeholder="Job title"
            stretch={false}
            value=""
            setValue={() => null}
            variant="v4"
          />
        </Stack>
        <Stack orientation="horizontal" gap="1" align="baseline">
          <Text variant="v4">At (Company)</Text>
          <Input
            placeholder="Company name"
            stretch={false}
            value=""
            setValue={() => null}
            variant="v4"
          />
        </Stack>
        <Stack orientation="horizontal" gap="1" align="baseline">
          <Text variant="v4">Apply by sending email to</Text>
          <Input
            placeholder="Your contact email"
            stretch={false}
            value=""
            setValue={() => null}
            variant="v4"
          />
        </Stack>
        <Stack orientation="horizontal" margin="2 0 0" align="center" gap="1">
          <Button
            variant="v3"
            onClick={() => alert("Stay updated at github.com/algeriastartupjobs")}
            vtName="new-post"
          >
            Post now
          </Button>
          <Text variant="v4">or</Text>
          <Link
            to="#"
            variant="v4"
            onClick={() => alert("Stay updated at github.com/algeriastartupjobs")}
          >
            Add more details
          </Link>
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
