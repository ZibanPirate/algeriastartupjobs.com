import { shouldAnimate } from "./should-animate";

let viewTransitionOneTimeSubscribers: Array<() => void> = [];

export const viewTransitionSubscribeOnce = (callback: () => void) => {
  viewTransitionOneTimeSubscribers.push(callback);

  return () => {
    viewTransitionOneTimeSubscribers = viewTransitionOneTimeSubscribers.filter(
      (subscriber) => subscriber !== callback
    );
  };
};

export const viewTransition = (callback: () => void, options = { skip: false }) => {
  const callbackWithSubscribers = () => {
    viewTransitionOneTimeSubscribers.forEach((subscriber) => subscriber());
    viewTransitionOneTimeSubscribers = [];

    return callback();
  };
  const disableAnimation = options.skip || !shouldAnimate();
  if (document.startViewTransition && !disableAnimation)
    document.startViewTransition(callbackWithSubscribers);
  else callbackWithSubscribers();
};
