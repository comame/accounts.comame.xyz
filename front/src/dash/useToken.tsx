let token = ""

async function checkToken(token: string): Promise<boolean> {
    const json = await fetch("/dash/rp/list", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token }),
    }).then((res) => res.json())
    return json.message !== "unauthorized"
}

export function useToken(): string {
    if (token) {
        return token
    }

    const tokenInUrl =
        sessionStorage.getItem("dash-token") || location.hash.slice(1)
    throw checkToken(tokenInUrl).then((result) => {
        if (result) {
            sessionStorage.setItem("dash-token", tokenInUrl)
            token = tokenInUrl
            location.hash = ""
        } else {
            sessionStorage.removeItem("dash-token")
            location.replace("/dash/signin")
            // Resolve しないことで再レンダリングを防ぐ
            return new Promise(() => {})
        }
    })
}
