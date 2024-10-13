import { useEffect } from "react";

interface UsePageTitleOptions {
  suffix?: string;
  enabled?: boolean;
}

export const usePageTitle = (
  title: string,
  { enabled = true, suffix = " | DZ Job" }: UsePageTitleOptions = {
    suffix: " | DZ Job",
    enabled: true,
  }
) => {
  useEffect(() => {
    if (!enabled) return;
    document.title = `${title}${suffix}`;
  }, [title, enabled, suffix]);
};
