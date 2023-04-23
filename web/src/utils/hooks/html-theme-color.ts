import { useEffect } from "react";

export const useHtmlThemeColor = () => {
  useEffect(() => {
    const matchMedia = window.matchMedia("(prefers-color-scheme: dark)");

    const applyHtmlThemeColor = (event: Pick<MediaQueryListEvent, "matches">) =>
      document
        .querySelector('meta[name="theme-color"]')
        ?.setAttribute("content", event.matches ? "#242424" : "#e8e8e8");

    applyHtmlThemeColor({ matches: matchMedia.matches });

    matchMedia.addEventListener("change", applyHtmlThemeColor);
    return () => matchMedia.removeEventListener("change", applyHtmlThemeColor);
  }, []);
};
