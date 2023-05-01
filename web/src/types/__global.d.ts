export {};

declare global {
  interface Window {
    __REDUX_DEVTOOLS_EXTENSION_COMPOSE__?: any;
    // https://developers.google.com/analytics/devguides/collection/analyticsjs/command-queue-reference#adding-commands-to-the-queue
    ga?: (command: string, ...fields: Array<string>) => void;
  }
}
