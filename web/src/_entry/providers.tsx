import { FC, PropsWithChildren } from "react";
import { RouterProvider } from "src/_routes";

export const Providers: FC<PropsWithChildren> = ({ children }) => {
  return <RouterProvider>{children}</RouterProvider>;
};
