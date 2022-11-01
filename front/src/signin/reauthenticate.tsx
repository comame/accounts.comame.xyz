import React, { useEffect, useRef, useState } from "react"
import { createRoot } from "react-dom/client"

import { Button, TextField } from "@charcoal-ui/react"
import {
    Layout,
    LayoutItem,
    LayoutItemHeader,
    LayoutItemBody,
} from "@charcoal-ui/react-sandbox"
import { Themed } from "../theme"
import { useContinueForm } from "./useContinueForm"
import { useQueryParams } from "./useQueryParams"
import {
    Bold,
    TextContainer,
    InputContainer,
    ButtonsContainer,
    Global,
} from "./layouts"
import { getUserAgentId } from "./getUserAgentId"
import { useRequiredInputElement } from "./useRequiredInputElement"

const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const [loginType, setLoginType] = useState("")
    const [hidden, setHidden] = useState(true)

    const [ContinueForm, _ref, next] = useContinueForm(
        csrfToken,
        loginType,
        stateId ?? undefined,
        relyingPartyId
    )

    const [invalidCredential, setInvalidCredential] = useState(false)
    const [sendingPassword, setSendingPassword] = useState(false)
    const [isEmpty, setIsEmpty] = useState(false)

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

    const formRef = useRef<HTMLFormElement>(null)
    const passwordRef = useRef<HTMLInputElement & HTMLTextAreaElement>(null)
    useRequiredInputElement([hidden])

    const [id, setId] = useState("")
    const [password, setPassword] = useState("")

    useEffect(() => {
        if (!hidden) {
            passwordRef.current?.focus()
        }
    }, [hidden])

    const onSubmitPassword = async (e: React.FormEvent) => {
        e.preventDefault()

        const valid = formRef.current?.reportValidity()

        if (!valid || !password) {
            setIsEmpty(true)
            return
        }

        setSendingPassword(true)

        const body = JSON.stringify({
            user_id: id,
            password,
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
        })
        const res = await fetch("/api/signin-password", {
            method: "POST",
            body,
            headers: {
                "Content-Type": "application/json",
            },
        })
        if (res.status !== 200) {
            setInvalidCredential(true)
            setSendingPassword(false)
            passwordRef.current?.focus()
            return
        }

        setLoginType("password")
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
                <Layout center wide>
                    <LayoutItem span={3}>
                        <LayoutItemHeader>
                            <div>
                                <Bold>{relyingPartyId}</Bold> にログイン
                            </div>
                        </LayoutItemHeader>
                        <LayoutItemBody>
                            <form ref={formRef}>
                                <TextContainer>
                                    <div>
                                        <Bold>{id}</Bold> さん
                                    </div>
                                    <div>
                                        続けるには、パスワードを入力してください
                                    </div>
                                </TextContainer>
                                <InputContainer>
                                    <TextField
                                        label="パスワード"
                                        placeholder="パスワード"
                                        type="password"
                                        required
                                        onChange={(e) => {
                                            setInvalidCredential(false)
                                            setPassword(e)
                                            setIsEmpty(false)
                                        }}
                                        invalid={invalidCredential || isEmpty}
                                        assistiveText={
                                            invalidCredential
                                                ? "パスワードが正しくありません"
                                                : isEmpty
                                                ? "パスワードを入力してください"
                                                : undefined
                                        }
                                        ref={passwordRef}
                                    ></TextField>
                                </InputContainer>
                                <ButtonsContainer>
                                    <Button
                                        variant="Primary"
                                        fixed
                                        onClick={onSubmitPassword}
                                        type="submit"
                                        disabled={sendingPassword}
                                    >
                                        続ける
                                    </Button>
                                    <Button
                                        fixed
                                        onClick={chooseOtherAccount}
                                        disabled={sendingPassword}
                                    >
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
