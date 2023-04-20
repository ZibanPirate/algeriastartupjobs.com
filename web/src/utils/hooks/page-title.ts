import { useEffect } from "react";

export const usePageTitle = (
  title: string,
  suffix = " | Algeria Startup Jobs"
) => {
  useEffect(() => {
    document.title = title + suffix;
  }, [title]);
};
