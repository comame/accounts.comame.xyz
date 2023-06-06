import { apis, request, response } from "./types"

export async function fetchApi<T extends keyof apis>(
    endpoint: T,
    body: request<T>
): Promise<response<T>> {
    return fetch(endpoint, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
    }).then((res) => res.json())
}
