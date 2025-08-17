const api = {
  get: async function <T>(url: string): Promise<T> {
    const response = await fetch(url, {
      method: "GET",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      throw new Error(
        `GET request failed: ${response.status} ${response.statusText}`
      );
    }

    return response.json() as T;
  },

  patch: async function <T>(url: string, data: unknown): Promise<T> {
    const response = await fetch(url, {
      method: "PATCH",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      throw new Error(
        `PATCH request failed: ${response.status} ${response.statusText}`
      );
    }

    return response.json() as T;
  },

  post: async function <T>(url: string, data: unknown): Promise<T> {
    const response = await fetch(url, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      throw new Error(
        `POST request failed: ${response.status} ${response.statusText}`
      );
    }

    return response.json() as T;
  },
};

export default api;
