import React, { useEffect, useState } from "react"
import { createRoot } from "react-dom/client"

import { Button } from "@charcoal-ui/react"
import { Themed } from "../theme"
import { useContinueForm } from "./useContinueForm"
import { useQueryParams } from "./useQueryParams"
import {
    Layout,
    LayoutItem,
    LayoutItemBody,
    LayoutItemHeader,
} from "@charcoal-ui/react-sandbox"
import { Bold, TextContainer, ButtonsContainer, Global } from "./layouts"
import { getUserAgentId } from "./getUserAgentId"

const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const [hidden, setHidden] = useState(true)

    const [ContinueForm, _ref, next] = useContinueForm(
        csrfToken,
        "consent",
        stateId ?? undefined,
        relyingPartyId
    )

    const [isSending, setIsSending] = useState(false)

    useEffect(() => {
        fetch("/api/signin-session", {
            method: "POST",
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                csrf_token: csrfToken,
                relying_party_id: relyingPartyId,
                user_agent_id: getUserAgentId(),
            }),
        })
            .then((res) => res.json())
            .then((json) => {
                if (json["user_id"]) {
                    setId(json["user_id"])
                    setHidden(false)
                } else {
                    location.replace(
                        `/signin?sid=${stateId}&cid=${encodeURIComponent(
                            relyingPartyId
                        )}`
                    )
                }
            })
    }, [])

    const [id, setId] = useState("")

    const onSubmit = async (e: React.FormEvent) => {
        setIsSending(true)
        e.preventDefault()
        next()
    }

    const chooseOtherAccount = (e: React.FormEvent) => {
        e.preventDefault()
        const continueUrl = encodeURIComponent(
            `${location.origin}/signin?sid=${stateId}&cid=${encodeURIComponent(
                relyingPartyId
            )}`
        )
        location.replace(`/signout?continue=${continueUrl}`)
    }

    return (
        <Themed>
            {!hidden && (
                <Layout wide center>
                    <LayoutItem span={3}>
                        <LayoutItemHeader>
                            <div>
                                <Bold>{relyingPartyId}</Bold> ???????????????
                            </div>
                        </LayoutItemHeader>
                        <LayoutItemBody>
                            <form>
                                <TextContainer>
                                    <div>
                                        <Bold>{id}</Bold> ??????
                                    </div>
                                    <div>?????????????????????</div>
                                </TextContainer>
                                <ButtonsContainer>
                                    <Button
                                        variant="Primary"
                                        fixed
                                        onClick={onSubmit}
                                        disabled={isSending}
                                        type="submit"
                                        autoFocus
                                    >
                                        ?????????
                                    </Button>
                                    <Button
                                        fixed
                                        onClick={chooseOtherAccount}
                                        disabled={isSending}
                                    >
                                        ?????????????????????????????????
                                    </Button>
                                </ButtonsContainer>
                            </form>
                        </LayoutItemBody>
                    </LayoutItem>
                </Layout>
            )}
            <ContinueForm />
            <Global />
        </Themed>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
