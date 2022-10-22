import { useEffect, useState } from "react"

export function useApi(
    token: string,
    endpoint: string,
    callback: (json: any) => void | Promise<void>
): [ call: (body?: any) => void ] {
    return [(body = {}) => {
        if (!token) {
            return
        }
        fetch(endpoint, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ token, ...body })
        }).then(res => res.json()).then(json => {
            if (json.message == 'unauthorized') {
                location.replace('/dash')
            } else {
                callback(json)
            }
        })
    }]
}
