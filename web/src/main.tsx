import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { EntryPoint } from "src/_entry";

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <EntryPoint />
  </StrictMode>
);
