export async function getRequest<T>(url: string): Promise<T> {
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
}

export async function patchRequest<T>(url: string, data: unknown): Promise<T> {
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
}
