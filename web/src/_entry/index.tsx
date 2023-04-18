import { FC, StrictMode } from "react";
import "./style.css";
import { Providers } from "./providers";
import { App } from "./app";
import { createRoot } from "react-dom/client";

const EntryPoint: FC = () => (
  <Providers>
    <App />
  </Providers>
);

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <EntryPoint />
  </StrictMode>
);
