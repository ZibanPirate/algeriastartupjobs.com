export const onceAtATime = <T extends (...args: any[]) => Promise<void>>(func: T): T => {
  let running = false;
  let latestArgs: Parameters<T> | null = null;
  return async function (...args: Parameters<T>): Promise<void> {
    if (running) {
      latestArgs = args;
      return;
    }
    running = true;
    await func(...args);
    running = false;
    if (latestArgs) {
      await func(...latestArgs);
      latestArgs = null;
    }
  } as T;
};
