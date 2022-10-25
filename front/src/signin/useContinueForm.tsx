import React, { useEffect, useRef, useState } from "react"
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

            const body = `csrf_token=${encodeURIComponent(
                csrfToken
            )}&login_type=${encodeURIComponent(
                loginType
            )}&state_id=${encodeURIComponent(
                stateId
            )}&relying_party_id=${encodeURIComponent(relyingPartyId!)}&user_agent_id=${encodeURIComponent(userAgentId)}`

            console.log("fetch")

            fetch("/api/signin-continue", {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded",
                },
                body,
                credentials: "include",
            })
                .then((res) => res.json())
                .then((json) => {
                    if (json.location) {
                        location.replace(json.location)
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
