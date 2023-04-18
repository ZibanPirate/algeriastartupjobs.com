import { FC, PropsWithChildren, useState } from "react";
import { RouterProvider as SRP, createBrowserRouter } from "react-router-dom";

// NOTE-ZM: this is a workaround for the stupid v6 idea of "react-router-dom"
export const RouterProvider: FC<PropsWithChildren> = ({ children }) => {
  const [router] = useState(
    createBrowserRouter([{ path: "*", element: children }])
  );
  return <SRP router={router} />;
};
