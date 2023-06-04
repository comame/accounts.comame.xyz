import { useSyncExternalStore } from "react";
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

type useSuspendApiReturnType<T extends keyof apis> = { data: response<T> };
export function useSuspendApi<T extends keyof apis>(
  token: string,
  endpoint: T,
  body: request<T>
): useSuspendApiReturnType<T> {
  const key = endpoint + "?key=" + JSON.stringify(body);

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
          fetching.delete(key);
          store.set(key, json);
        }
      });

  const rStore = useSyncExternalStore(store.subscribe, store.getSnapshot);
  const cached = rStore.get(key);

  if (cached) {
    return {
      data: cached,
    };
  } else if (fetching.get(key)) {
    throw new Promise(() => {});
  } else {
    fetching.set(key, true);
    throw fetcher(body);
  }
}

export function mutateAll() {
  store.clear();
}

type cacheMapT = Map<string, any>;

const store = {
  _self: new Map() as cacheMapT,
  _listeners: [] as (() => unknown)[],

  set(key: string, value: any) {
    store._self = new Map(store._self.set(key, value));
    store._dispatch();
  },
  clear() {
    store._self = new Map();
    store._dispatch();
  },

  _dispatch() {
    for (const listener of store._listeners) {
      listener();
    }
  },
  subscribe(listener: () => unknown) {
    store._listeners = [...store._listeners, listener];
    return () => {
      store._listeners = store._listeners.filter((l) => l != listener);
    };
  },
  getSnapshot() {
    return store._self;
  },
};

const fetching: Map<string, true> = new Map();
