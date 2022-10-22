import { useEffect, useState } from "react"

const useApiEffect = (token: string, effect: () => void, deps: any[]) => {
    useEffect(() => {
        if (token) {
            effect()
        }
    }, [token, ...deps])
}

export function useApi(
    token: string,
    endpoint: string,
    body: any,
    callback: (json: any) => void | Promise<void>
): [ reload: () => void ] {
    const [reloadFlg, setReloadFlg] = useState(true)
    useApiEffect(token, () => {
        fetch(endpoint, {
            method: 'POST',
            body: JSON.stringify(body),
        }).then(res => res.json()).then(json => {
            if (json.message == 'unauthorized') {
                location.replace('/dash')
            }
            callback(json)
        })
    }, [endpoint, reloadFlg])

    return [ () => {
        setReloadFlg(v => !v)
    }]
}
