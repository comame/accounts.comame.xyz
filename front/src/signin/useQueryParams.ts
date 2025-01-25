import { useMemo } from "react"

export function useQueryParams() {
    const relyingPartyId = useMemo(
        () =>
            decodeURIComponent(
                new URL(location.href).searchParams.get("cid") ?? "",
            ),
        [],
    )

    return { relyingPartyId }
}
