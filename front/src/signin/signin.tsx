import React, { useEffect, useMemo, useRef, useState } from "react"
import { createRoot } from "react-dom/client"

import { Button, TextField } from "@charcoal-ui/react"
import { Themed } from "../theme"
import { useContinueForm } from "./useContinueForm"
import { useQueryParams } from "./useQueryParams"
import {
    Layout,
    LayoutItem,
    LayoutItemBody,
    LayoutItemHeader,
} from "@charcoal-ui/react-sandbox"
import { Bold, ButtonsContainer, Global, InputContainer } from "./layouts"
import { getUserAgentId } from "./getUserAgentId"
import { useRequiredInputElement } from "./useRequiredInputElement"

const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const hash = useMemo(() => location.hash.slice(1), [])
    const maxage = useMemo(
        () =>
            Number.parseInt(
                new URL(location.href).searchParams.get("age") ?? "0",
                10
            ),
        []
    )

    const failRef = useRef<HTMLFormElement>(null)

    const [loginType, setLoginType] = useState("")
    const [hidden, setHidden] = useState(true)

    const [ContinueForm, _ref, next] = useContinueForm(
        csrfToken,
        loginType,
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
                relying_party_id: relyingPartyId,
                user_agent_id: getUserAgentId(),
            }),
        })
            .then((res) => {
                if (res.status !== 200 && hash === "nointeraction") {
                    return Promise.reject("nointeraction-fail")
                } else if (res.status !== 200) {
                    return Promise.reject("no-session")
                }
                return res.json()
            })
            .catch((reason) => {
                if (reason === "nointeraction-fail") {
                    failRef.current?.submit()
                } else if (reason === "no-session") {
                    setHidden(false)
                }
            })
            .then((json) => {
                if (!json) {
                    return
                }
                if (hash !== "maxage") {
                    // setLoginType("session")
                    // next()
                    location.replace(
                        `/confirm?sid=${stateId}&cid=${encodeURIComponent(
                            relyingPartyId
                        )}`
                    )
                } else {
                    const lastAuth = json["last_auth"]
                    const now = Math.trunc(Date.now() / 1000)
                    if (now <= lastAuth + maxage - 3 /* lag */) {
                        setLoginType("session")
                        next()
                    } else {
                        location.replace(
                            `/reauthenticate?sid=${stateId}&cid=${encodeURIComponent(
                                relyingPartyId
                            )}`
                        )
                    }
                }
            })
    }, [])

    const [id, setId] = useState("")
    const [password, setPassword] = useState("")

    const formRef = useRef<HTMLFormElement>(null)
    const idRef = useRef<HTMLInputElement & HTMLTextAreaElement>(null)
    const passwordRef = useRef<HTMLInputElement & HTMLTextAreaElement>(null)
    useRequiredInputElement([hidden])

    useEffect(() => {
        if (!hidden) {
            idRef.current?.focus()
        }
    }, [hidden])

    const [sendingPassword, setSendingPassword] = useState(false)
    const [invalidCredential, setInvalidCredential] = useState(false)
    const [isEmptyId, setIsEmptyId] = useState(false)
    const [isEmptyPassword, setIsEmptyPassword] = useState(false)

    const onSubmitPassword = async (e: React.FormEvent) => {
        e.preventDefault()

        const valid = formRef.current?.reportValidity()

        if (!id) {
            setIsEmptyId(true)
        }
        if (!password) {
            setIsEmptyPassword(true)
        }
        if (!valid || !id || !password) {
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
                                <InputContainer>
                                    <TextField
                                        showLabel
                                        label="ID"
                                        required
                                        onChange={(e) => {
                                            setInvalidCredential(false)
                                            setIsEmptyId(false)
                                            setId(e)
                                        }}
                                        invalid={invalidCredential || isEmptyId}
                                        assistiveText={
                                            invalidCredential
                                                ? "ID またはパスワードが正しくありません"
                                                : isEmptyId
                                                ? "ID を入力してください"
                                                : undefined
                                        }
                                        ref={idRef}
                                    ></TextField>
                                    <TextField
                                        showLabel
                                        label="パスワード"
                                        type="password"
                                        required
                                        onChange={(e) => {
                                            setInvalidCredential(false)
                                            setIsEmptyPassword(false)
                                            setPassword(e)
                                        }}
                                        invalid={
                                            invalidCredential || isEmptyPassword
                                        }
                                        assistiveText={
                                            invalidCredential
                                                ? "ID またはパスワードが正しくありません"
                                                : isEmptyPassword
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
                                        ログイン
                                    </Button>
                                </ButtonsContainer>
                            </form>
                        </LayoutItemBody>
                    </LayoutItem>
                </Layout>
            )}
            <ContinueForm />
            <form
                action="/api/signin-continue-nointeraction-fail"
                method="POST"
                encType="application/x-www-form-urlencoded"
                target="_self"
                ref={failRef}
            >
                <input
                    name="csrf_token"
                    type="hidden"
                    value={csrfToken}
                ></input>
                <input
                    name="state_id"
                    type="hidden"
                    value={stateId ?? ""}
                ></input>
            </form>
            <Global />
        </Themed>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
