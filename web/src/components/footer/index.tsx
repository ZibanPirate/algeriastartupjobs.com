import { FC } from "react";
import { Stack } from "../stack";
import { Text } from "../text";
import { Link } from "../link";

export const Footer: FC = () => {
  return (
    <Stack orientation="vertical" margin="auto" align="center" padding="1">
      <Text variant="v4">
        Source code is publicly available at&nbsp;
        <Link to="https://github.com/zibanpirate/dzjob" variant="v4">
          Github
        </Link>
        , stay tuned at{" "}
        <Link to="https://twitter.com/algeriastartup" variant="v4">
          Twitter
        </Link>
        ,{" "}
        <Link to="https://www.instagram.com/algeriastartup" variant="v4">
          Instagram
        </Link>{" "}
        and{" "}
        <Link to="https://www.facebook.com/profile.php?id=100093094762345" variant="v4">
          Facebook
        </Link>
        .
      </Text>
    </Stack>
  );
};
