import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";

export default (): ReturnType<FC> => {
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
        </Stack>
      </div>
    </Stack>
  );
};
