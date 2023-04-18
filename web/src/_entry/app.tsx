import { FC } from "react";
import "./style.css";
import { Route, Routes } from "react-router-dom";
import { LazyPages } from "src/pages";

export const App: FC = () => {
  return (
    <Routes>
      <Route path="/" element={LazyPages["landing"]} />
      <Route path="*" element={LazyPages["404"]} />
    </Routes>
  );
};
