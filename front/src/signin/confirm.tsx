import React, { useEffect, useState } from "react"
import { createRoot } from "react-dom/client"

import { Button } from "@charcoal-ui/react"
import { Themed } from "../theme"
import { useQueryParams } from "./useQueryParams"
import {
    Layout,
    LayoutItem,
    LayoutItemBody,
    LayoutItemHeader,
} from "@charcoal-ui/react-sandbox"
import { Bold, TextContainer, ButtonsContainer, Global } from "./layouts"
import { getUserAgentId } from "./getUserAgentId"
import { fetchApi } from "./fetchApi"

const App = () => {
    const { stateId, relyingPartyId, csrfToken, userId } = useQueryParams()

    const [isSending, setIsSending] = useState(false)

    const onSubmit = async (e: React.FormEvent) => {
        setIsSending(true)
        e.preventDefault()

        const res = await fetchApi("/api/signin-session", {
            state_id: stateId ?? "",
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
        })

        if ("error" in res) {
            window.alert(res.error)
            return
        }

        location.replace(res.location)
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
            <Layout wide center>
                <LayoutItem span={3}>
                    <LayoutItemHeader>
                        <div>
                            <Bold>{relyingPartyId}</Bold> にログイン
                        </div>
                    </LayoutItemHeader>
                    <LayoutItemBody>
                        <form>
                            <TextContainer>
                                <div>
                                    <Bold>{userId}</Bold> さん
                                </div>
                                <div>続行しますか？</div>
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
                                    続ける
                                </Button>
                                <Button
                                    fixed
                                    onClick={chooseOtherAccount}
                                    disabled={isSending}
                                >
                                    アカウントを切り替える
                                </Button>
                            </ButtonsContainer>
                        </form>
                    </LayoutItemBody>
                </LayoutItem>
            </Layout>
            <Global />
        </Themed>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
