import { FC } from "react";
import { Search, SearchProps } from "src/components/search";

interface GlobalSearchProps extends SearchProps {}

export const GlobalSearch: FC<GlobalSearchProps> = ({ ...props }) => {
  return <Search {...props} placeholder="Search 1354 jobs" vtName="global-search" />;
};
