import { FC, PropsWithChildren, useEffect, useState } from "react";
import { RouterProvider as SRP, createBrowserRouter, useLocation } from "react-router-dom";

// NOTE-ZM: this is a workaround for the stupid idea of v6 of "react-router-dom"
export const RouterProvider: FC<PropsWithChildren> = ({ children }) => {
  const [router] = useState(createBrowserRouter([{ path: "*", element: children }]));
  return <SRP router={router} />;
};

let navigatedWithinWebsiteAtLeastOnce = false;
let initialHistoryLength = window.history.length;

/**
 * We check if we ever navigated withing the websites, at least once.
 * If we did, that means navigating back will still keep us in the website.
 */
export const isNavigatingBackLeavesWebsite = () => !navigatedWithinWebsiteAtLeastOnce;

export const LocationListenerProvider: FC<PropsWithChildren> = ({ children }) => {
  let location = useLocation();

  useEffect(() => {
    navigatedWithinWebsiteAtLeastOnce = window.history.length > initialHistoryLength;

    // @TODO-ZM: capture page views here
  }, [location]);
  return <>{children}</>;
};
