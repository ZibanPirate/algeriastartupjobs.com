import { useEffect } from "react";

interface UsePageTitleOptions {
  suffix?: string;
  enabled?: boolean;
}

export const usePageTitle = (
  title: string,
  { enabled = true, suffix = " | Algeria Startup Jobs" }: UsePageTitleOptions = {
    suffix: " | Algeria Startup Jobs",
    enabled: true,
  }
) => {
  useEffect(() => {
    if (!enabled) return;
    document.title = `🇩🇿 ${title}${suffix}`;
  }, [title, enabled, suffix]);
};
