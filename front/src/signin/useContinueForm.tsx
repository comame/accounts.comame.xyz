import React, { useEffect, useRef, useState } from "react"
import { fetchApi } from "./fetchApi"
import { getUserAgentId } from "./getUserAgentId"

export function useContinueForm(
    csrfToken: string | undefined,
    loginType: string | undefined,
    stateId: string | undefined,
    relyingPartyId: string | undefined
): [node: React.FC, ref: React.RefObject<HTMLFormElement>, next: () => void] {
    const ref = useRef<HTMLFormElement>(null)

    const [next, setNext] = useState(false)

    // Delays 1 tick
    const onSubmit = () => {
        setNext(true)
    }

    useEffect(() => {
        if (next) {
            const csrfToken = ref.current?.csrf_token.value
            const loginType = ref.current?.login_type.value
            const stateId = ref.current?.state_id.value
            const userAgentId = getUserAgentId()

            fetchApi("/api/signin-continue", {
                csrf_token: csrfToken,
                login_type: loginType,
                state_id: stateId,
                relying_party_id: relyingPartyId!,
                user_agent_id: userAgentId,
            }).then((res) => {
                if ("error" in res) {
                    if (res.error === "no_permission") {
                        handleNoPermission(relyingPartyId!)
                    } else {
                        window.alert(res.error)
                    }
                } else {
                    location.replace(res.location)
                }
            })
        }
    }, [next])

    const element = () => (
        <form
            ref={ref}
            action="/api/signin-continue"
            method="POST"
            encType="application/x-www-form-urlencoded"
            target="_self"
        >
            <input name="csrf_token" type="hidden" value={csrfToken}></input>
            <input name="login_type" type="hidden" value={loginType}></input>
            <input name="state_id" type="hidden" value={stateId}></input>
            <input
                name="relying_party_id"
                type="hidden"
                value={relyingPartyId}
            ></input>
        </form>
    )

    return [element, ref, onSubmit]
}

async function handleNoPermission(clientId: string) {
    const { html } = await import("./accessDeniedHtml")
    document.title = "Access Denied"
    document.body.innerHTML = html.replace("$RP", clientId)
}
