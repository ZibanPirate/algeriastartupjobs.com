import { FC } from "react";
import { Search, SearchProps } from "src/components/search";

interface GlobalSearchProps extends SearchProps {
  total_post_count: number;
}

export const GlobalSearch: FC<GlobalSearchProps> = ({ total_post_count, ...props }) => {
  return (
    <Search
      {...props}
      placeholder={`Search${total_post_count > 0 ? ` ${total_post_count} ` : " "}jobs`}
      vtName="global-search"
    />
  );
};
