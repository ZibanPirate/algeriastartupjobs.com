export const viewTransition = (callback: () => void, options = { skip: false }) => {
  const disableAnimation = options.skip || matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (document.startViewTransition && !disableAnimation) document.startViewTransition(callback);
  else callback();
};
