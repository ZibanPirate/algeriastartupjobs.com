import { FC, PropsWithChildren, useEffect, useState } from "react";
import {
  RouterProvider as SRP,
  createBrowserRouter,
  useLocation,
  useNavigationType,
} from "react-router-dom";

let browserRouter: ReturnType<typeof createBrowserRouter>;
export const getBrowserRouter = () => browserRouter;

// NOTE-ZM: this is a workaround for the stupid idea of v6 of "react-router-dom"
export const RouterProvider: FC<PropsWithChildren> = ({ children }) => {
  const [router] = useState(createBrowserRouter([{ path: "*", element: children }]));
  useEffect(() => {
    browserRouter = router;
  }, [router]);

  return <SRP router={router} />;
};

let navigatedWithinWebsiteAtLeastOnce = false;
let initialHistoryLength = window.history.length;
const urlsHistory: string[] = [location.pathname];

/**
 * We check if we ever navigated withing the websites, at least once.
 * If we did, that means navigating back will still keep us in the website.
 */
export const isNavigatingBackLeavesWebsite = () => !navigatedWithinWebsiteAtLeastOnce;

export const getBeforeLastURLPathname = () => urlsHistory[urlsHistory.length - 2];

export const LocationListenerProvider: FC<PropsWithChildren> = ({ children }) => {
  let location = useLocation();
  const navigationType = useNavigationType();

  useEffect(() => {
    navigatedWithinWebsiteAtLeastOnce = window.history.length > initialHistoryLength;

    switch (navigationType) {
      case "POP":
        if (urlsHistory.length > 1) urlsHistory.pop();
        break;
      case "PUSH":
        urlsHistory.push(location.pathname);
        break;
      case "REPLACE":
        urlsHistory[urlsHistory.length - 1] = location.pathname;
        break;
    }

    if (window.ga) {
      window.ga("set", "page", location.pathname);
      window.ga("send", "pageview");
    }
  }, [location, navigationType]);
  return <>{children}</>;
};
