import { getConfig } from "../config/get-config";

type CreateFetchStreamParams = {
  url: string;
  baseURL?: string;
};

export const createFetchStream = <R extends Record<string, unknown>>({
  url,
  baseURL = getConfig().api.base_url,
}: CreateFetchStreamParams) => {
  let eventSource: EventSource | null = null;
  let onMessage: (
    param: { response: R | null; error?: never } | { error: Error; response?: never }
  ) => void = () => {};

  return {
    /**
     * NOTE: this will not throw on network error, it will instead return null
     */
    listen: async (): Promise<R | null> => {
      const promise = new Promise<R | null>((resolve, reject) => {
        onMessage = ({ response, error }) => {
          if (error) reject(error);
          else resolve(response);
          onMessage = () => {};
        };
      });

      if (!eventSource) {
        const normalizedURL = url.startsWith("/") ? url : `/${url}`;
        eventSource = new EventSource(`${baseURL}${normalizedURL}`);

        eventSource;

        eventSource.addEventListener("error", (ev) => {
          if (ev.eventPhase === EventSource.CLOSED) {
            onMessage({ response: null });
            eventSource?.close();
            return;
          }

          eventSource?.close();
          onMessage({ error: new Error(`EventSource error: ${ev}`) });
        });

        eventSource.addEventListener("message", (ev) => {
          try {
            const data = JSON.parse(ev.data) as R;
            onMessage({ response: data });
          } catch (error) {
            onMessage({ error: new Error(`EventSource error: ${error}`) });
            eventSource?.close();
          }
        });

        eventSource.addEventListener("", () => {});
      }

      return promise;
    },
    close: () => {
      eventSource?.close();
      eventSource = null;
    },
  };
};
