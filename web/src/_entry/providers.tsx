import { FC, PropsWithChildren } from "react";
import { Provider as StateProvider } from "react-redux";
import { RouterProvider } from "src/components/router-provider";
import { getStore } from "src/state";

export const Providers: FC<PropsWithChildren> = ({ children }) => {
  return (
    <StateProvider store={getStore()}>
      <RouterProvider>{children}</RouterProvider>
    </StateProvider>
  );
};
