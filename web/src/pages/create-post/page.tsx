import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { Input } from "src/components/input";
import { Button } from "src/components/button";
import { Select } from "src/components/select";
import { Account } from "src/models/account";
import { useSliceSelector } from "src/utils/state/selector";
import { getStateActions } from "src/state";

export const Page: FC = () => {
  usePageTitle("Post a job add for free!");

  const { title, poster_type, poster_name, poster_contact } = useSliceSelector("createPostPage");
  const { set } = getStateActions().createPostPage;

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
            value={title}
            setValue={(value) => set({ title: value })}
            variant="v4"
          />
        </Stack>
        <Stack orientation="horizontal" gap="1" align="baseline">
          <Select<Account["type"]>
            variant="v4"
            padding="0"
            value={poster_type}
            setValue={(value) => set({ poster_type: value })}
            options={[
              { value: "Company", label: "At (Company)" },
              { value: "Individual", label: "By (Individual)" },
            ]}
          />
          <Input
            placeholder={`${poster_type} name`}
            stretch={false}
            value={poster_name}
            setValue={(value) => set({ poster_name: value })}
            variant="v4"
          />
        </Stack>
        <Stack orientation="horizontal" gap="1" align="baseline">
          <Text variant="v4">Candidate apply by sending email to</Text>
          <Input
            placeholder={`${poster_type} contact email`}
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
