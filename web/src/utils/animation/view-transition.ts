import { shouldAnimate } from "./should-animate";

export const viewTransition = (callback: () => void, options = { skip: false }) => {
  const disableAnimation = options.skip || !shouldAnimate();
  if (document.startViewTransition && !disableAnimation) document.startViewTransition(callback);
  else callback();
};
