import { useEffect, useState } from "react";

export const useComponentDidMount = (): boolean => {
  const [mounted, setMounted] = useState(false);
  useEffect(() => setMounted(true), []);
  return mounted;
};
