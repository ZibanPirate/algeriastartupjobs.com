import { FC } from "react";
import "./style.css";
import "src/utils/css/index.css";
import { Route, Routes } from "react-router-dom";
import { LazyPages } from "src/pages";
import { useHtmlThemeColor } from "src/utils/hooks/html-theme-color";
import { POST_PAGE_URL } from "src/utils/urls/common";

export const App: FC = () => {
  useHtmlThemeColor();
  return (
    <Routes>
      <Route path="/" element={LazyPages["landing"]} />
      <Route path={POST_PAGE_URL} element={LazyPages["post"]} />
      <Route path="*" element={LazyPages["404"]} />
    </Routes>
  );
};
