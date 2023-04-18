import { FC } from "react";
import "./style.css";
import { Providers } from "./providers";
import { App } from "./app";

export const EntryPoint: FC = () => (
  <Providers>
    <App />
  </Providers>
);
