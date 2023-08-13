import { FC, StrictMode, useEffect } from "react";
import "./style.css";
import { Providers } from "./providers";
import { App } from "./app";
import { createRoot } from "react-dom/client";
import * as Sentry from "@sentry/react";
import { getConfig } from "src/utils/config/get-config";
import { getStage } from "src/utils/config/get-stage";
import {
  createRoutesFromChildren,
  matchRoutes,
  useLocation,
  useNavigationType,
} from "react-router-dom";

const { stage } = getStage();
const { api, web } = getConfig();

const SENTRY_ON_DEVELOPMENT: boolean = false;

Sentry.init({
  dsn: "https://62bd4b5f29373523b20df4381ba81910@o4505697083457536.ingest.sentry.io/4505697169637376",
  integrations: [
    new Sentry.BrowserTracing({
      // Set 'tracePropagationTargets' to control for which URLs distributed tracing should be enabled
      tracePropagationTargets: [api.base_url],
      routingInstrumentation: Sentry.reactRouterV6Instrumentation(
        useEffect,
        useLocation,
        useNavigationType,
        createRoutesFromChildren,
        matchRoutes
      ),
    }),
    new Sentry.Replay(),
  ],
  // Performance Monitoring
  // @TODO-ZM: reduce once we have actual users
  tracesSampleRate: stage === "development" ? 1.0 : 1.0,
  // Session Replay
  // replaysSessionSampleRate: stage === "development" ? 1.0 : 0.1,
  // replaysOnErrorSampleRate: stage === "development" ? 1.0 : 0.1,
  //
  release: web.version,
  environment: stage,
  enabled: stage !== "development" || SENTRY_ON_DEVELOPMENT,
});

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
