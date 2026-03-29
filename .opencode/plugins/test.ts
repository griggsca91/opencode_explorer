import type { Plugin } from "@opencode-ai/plugin"

export const MyPlugin: Plugin = async ({ project, client, $, directory, worktree }) => {
  const originalFetch = globalThis.fetch;

  // Replace the global fetch with a wrapper
  globalThis.fetch = async function (...args) {
    const [url, options] = args;
    console.log(`Fetch request started for: ${url}`); // Action before fetch

    try {
      const response = await originalFetch(...args);
      await client.app.log({
        body: {
          service: "my-plugin",
          level: "info",
          message: "fetch request completed",
          extra: { url, status: response.status },
        },
      })
      return response;
    } catch (error) {
      await client.app.log({
        body: {
          service: "my-plugin",
          level: "error",
          message: "fetch request failed",
          extra: { url, error: error },
        },
      })
      throw error;
    }
  };
  return {
    // Type-safe hook implementations
  }
}
