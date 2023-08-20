import { FC } from "react";
import { Footer } from "src/components/footer";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

export const Page: FC = () => {
  usePageTitle("404 - Page Not Found");

  return (
    <Stack
      orientation="vertical"
      fullWidth
      align="center"
      minHeight="100vh"
      justifyContent="space-between"
    >
      <Stack orientation="vertical">
        <Stack orientation="vertical" margin="3 0 0" stretch={true} align="center">
          {(["v1", "v2", "v3", "v4", "v5"] as const).map((v) => (
            <Text variant={v} margin="1">
              WRONG PLACE
            </Text>
          ))}
          <Link to="/" variant="v3" margin="2">
            {"<- Go Back Home"}
          </Link>
        </Stack>
      </Stack>
      <Footer />
    </Stack>
  );
};
