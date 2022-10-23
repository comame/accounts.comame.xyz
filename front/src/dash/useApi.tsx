import { useEffect, useState } from "react"

export function useApi(
    token: string,
    endpoint: string,
    callback: (json: any) => void | Promise<void>
): [call: (body?: any) => Promise<void>] {
    return [
        async (body = {}) => {
            if (!token) {
                return Promise.resolve()
            }
            const res = await fetch(endpoint, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ token, ...body }),
            })
            const json = await res.json()
            if (json.message == "unauthorized") {
                location.replace("/dash")
            } else {
                callback(json)
            }
        },
    ]
}
