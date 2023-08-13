import { FC, PropsWithChildren } from "react";
import { Provider as StateProvider } from "react-redux";
import { ErrorBoundaryProvider } from "src/components/error-boundary-provider";
import { LocationListenerProvider, RouterProvider } from "src/components/router-provider";
import { getStore } from "src/state";

export const Providers: FC<PropsWithChildren> = ({ children }) => {
  return (
    <ErrorBoundaryProvider>
      <StateProvider store={getStore()}>
        <RouterProvider>
          <LocationListenerProvider>{children}</LocationListenerProvider>
        </RouterProvider>
      </StateProvider>
    </ErrorBoundaryProvider>
  );
};
