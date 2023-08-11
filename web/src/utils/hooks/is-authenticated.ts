import { useEffect, useState } from "react";
import { authSubscribe } from "../auth/subscribe";
import { authVerify } from "../auth/verify";

export const useIsAuthenticated = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(authVerify());

  useEffect(() => {
    return authSubscribe(() => {
      setIsAuthenticated(authVerify());
    });
  }, []);

  return { isAuthenticated };
};
