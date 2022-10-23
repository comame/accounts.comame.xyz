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

const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const [hidden, setHidden] = useState(true)

    const [ContinueForm, _ref, next] = useContinueForm(
        csrfToken,
        "consent",
        stateId ?? undefined,
        relyingPartyId
    )

    useEffect(() => {
        fetch("/api/signin-session", {
            method: "POST",
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                csrf_token: csrfToken,
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

    const [disabled, setDisabled] = useState(true)
    useEffect(() => {
        setTimeout(() => {
            setDisabled(false)
        }, 800)
    }, [])
    useEffect(() => {
        const button = document.getElementsByTagName("button")[0]
        if (!button) {
            return
        }
        if (!button.disabled) {
            button.focus()
        }
    }, [disabled])

    return (
        <Themed>
            {!hidden && (
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
                                        <Bold>{id}</Bold> さん
                                    </div>
                                    <div>続行しますか？</div>
                                </TextContainer>
                                <ButtonsContainer>
                                    <Button
                                        variant="Primary"
                                        fixed
                                        onClick={onSubmit}
                                        disabled={disabled}
                                        type="submit"
                                    >
                                        続ける
                                    </Button>
                                    <Button fixed onClick={chooseOtherAccount}>
                                        アカウントを切り替える
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
