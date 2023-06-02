import { useEffect, useState } from "react";
import { apis, request, response } from "./types";

export async function fetchApi<T extends keyof apis>(
  token: string,
  endpoint: T,
  body: request<T>
): Promise<response<T>> {
  const res = await fetch(endpoint, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ token, ...body }),
  });
  const json = await res.json();
  if (json.message == "unauthorized") {
    location.href = "/dash/signin";
    return new Promise(() => {});
  } else {
    return json;
  }
}

const suspendApiResponses: Map<string, any> = new Map();

type useSuspendApiReturnType<T extends keyof apis> = {
  data: response<T>;
  mutate: () => void;
};
export function useSuspendApi<T extends keyof apis>(
  token: string,
  endpoint: T,
  body: request<T>,
  key: string = endpoint
): useSuspendApiReturnType<T> {
  const fetcher = (body: any = {}) =>
    fetch(endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ token, ...body }),
    })
      .then((res) => res.json())
      .then((json) => {
        if (json.message == "unauthorized") {
          location.href = "/dash/signin";
          // 再レンダリングしない
          return new Promise(() => {});
        } else {
          suspendApiResponses.set(key, json);
        }
      });

  const cached = suspendApiResponses.get(key);

  const [_s, update] = useState(false);

  if (cached) {
    return {
      data: cached,
      mutate: () => {
        suspendApiResponses.delete(key);
        update((v) => !v);
      },
    };
  } else {
    throw fetcher(body);
  }
}

export function mutateAll() {
  const keys = suspendApiResponses.keys();
  for (const key of keys) {
    suspendApiResponses.delete(key);
  }
}
