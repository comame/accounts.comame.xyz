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
import { fetchApi } from "./fetchApi"

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

    const [loginType, setLoginType] = useState("")
    const [hidden, setHidden] = useState(true)

    const [ContinueForm, _ref, next] = useContinueForm(
        csrfToken,
        loginType,
        stateId ?? undefined,
        relyingPartyId
    )

    useEffect(() => {
        fetchApi("/api/signin-session", {
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
        }).then((res) => {
            if ("error" in res && res.error === "bad_request") {
                throw "bad_request"
            }

            if (hash === "nointeraction") {
                if ("error" in res && res.error === "no_session") {
                    fetchApi("/api/signin-continue-nointeraction-fail", {
                        csrf_token: csrfToken,
                        relying_party_id: relyingPartyId,
                        user_agent_id: getUserAgentId(),
                        state_id: stateId!,
                    }).then((res) => {
                        if ("error" in res) {
                            window.alert(res.error)
                        } else {
                            location.replace(res.location)
                        }
                    })
                } else {
                    setLoginType("session")
                    next()
                }
                return
            }

            if ("error" in res && res.error === "no_session") {
                setHidden(false)
                return
            }

            if ("error" in res) {
                throw "unreachable"
            }

            if (hash === "maxage") {
                const lastAuth = res.last_auth
                const now = Math.trunc(Date.now() / 1000)
                if (lastAuth && now <= lastAuth + maxage - 3 /* lag */) {
                    setLoginType("session")
                    next()
                } else {
                    location.replace(
                        `/reauthenticate?sid=${stateId}&cid=${encodeURIComponent(
                            relyingPartyId
                        )}`
                    )
                }
                return
            }

            // setLoginType("sessiozn")
            // next()
            location.replace(
                `/confirm?sid=${stateId}&cid=${encodeURIComponent(
                    relyingPartyId
                )}`
            )
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

        const res = await fetchApi("/api/signin-password", {
            user_id: id,
            password,
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
        })

        if ("error" in res && res.error === "invalid_credential") {
            setInvalidCredential(true)
            setSendingPassword(false)
            passwordRef.current?.focus()
            return
        }

        if ("error" in res && res.error === "bad_request") {
            throw "bad_request"
        }

        if ("error" in res) {
            throw "unreachable"
        }

        setLoginType("password")
        next()
    }

    const signinWithGoogle = async () => {
        if (!stateId) {
            throw "bad_request"
        }

        let result = await fetchApi("/signin/google", {
            state_id: stateId,
            user_agent_id: getUserAgentId(),
        })

        if ("error" in result) {
            alert("Error!")
            return
        }

        location.replace(result.location)
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
                                        variant="Default"
                                        fixed
                                        onClick={onSubmitPassword}
                                        type="submit"
                                        disabled={sendingPassword}
                                    >
                                        ログイン
                                    </Button>
                                    <Button
                                        variant="Primary"
                                        fixed
                                        onClick={signinWithGoogle}
                                        type="button"
                                    >
                                        Google でログイン
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
