let token = "";

async function checkToken(token: string): Promise<boolean> {
  const json = await fetch("/rp/list", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ token }),
  }).then((res) => res.json());
  return json.message !== "unauthorized";
}

export function useToken(): string {
  if (token) {
    return token;
  }

  const tokenInUrl = location.hash.slice(1);
  const tokenInStore = sessionStorage.getItem("dash-token");

  throw Promise.all([
    checkToken(tokenInStore ?? ""),
    checkToken(tokenInUrl),
  ]).then(([storeOk, urlOk]) => {
    if (!storeOk && !urlOk) {
      sessionStorage.removeItem("dash-token");
      location.replace("/signin");
      // Resolve しないことで再レンダリングを防ぐ
      return new Promise(() => {});
    }

    let localToken = "";
    if (storeOk) {
      localToken = tokenInStore as string;
    }
    if (urlOk) {
      localToken = tokenInUrl;
    }

    sessionStorage.setItem("dash-token", localToken);
    token = localToken;
    location.hash = "";
  });
}
