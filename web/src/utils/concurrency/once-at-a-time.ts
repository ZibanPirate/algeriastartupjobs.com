type OnceAtATimeOptions = {
  runLastCall?: boolean;
};

export const onceAtATime = <T extends (...args: any[]) => Promise<void>>(
  func: T,
  { runLastCall = false }: OnceAtATimeOptions = { runLastCall: false }
): T => {
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
    if (latestArgs && runLastCall) {
      await func(...latestArgs);
      latestArgs = null;
    }
  } as T;
};
