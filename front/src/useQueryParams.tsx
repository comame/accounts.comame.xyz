import { useMemo } from "react"

export function useQueryParams() {
    const csrfToken = useMemo(() =>
        (document.getElementById('csrf-token') as HTMLMetaElement).content,
    [])
    const relyingPartyId = useMemo(() =>
        decodeURIComponent(new URL(location.href).searchParams.get('cid') ?? ''),
    [])
    const stateId = useMemo(() =>
        new URL(location.href).searchParams.get('sid'),
    [])

    return { csrfToken, relyingPartyId, stateId }
}
