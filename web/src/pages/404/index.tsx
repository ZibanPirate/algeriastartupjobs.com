import { FC } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";

export default (): ReturnType<FC> => {
  return (
    <div className="absolute-center">
      <Stack orientation="vertical" align="center">
        {(["v1", "v2", "v3", "v4", "v5"] as const).map((v) => (
          <Text variant={v} margin="1">
            WRONG PLACE
          </Text>
        ))}
        <Link to="/" variant="v3" margin="2">
          {"<- Go Back Home"}
        </Link>
      </Stack>
    </div>
  );
};
