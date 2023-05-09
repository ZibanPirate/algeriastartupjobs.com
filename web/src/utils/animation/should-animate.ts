export const shouldAnimate = () => {
  const disableAnimation =
    window.innerWidth < 768 || matchMedia("(prefers-reduced-motion: reduce)").matches;

  return !disableAnimation;
};
