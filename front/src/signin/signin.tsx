import React, { useEffect, useMemo, useRef, useState } from "react"
import { createRoot } from "react-dom/client"

import { Button, TextField } from "@charcoal-ui/react"
import { Themed } from "../theme"
import { useQueryParams } from "./useQueryParams"
import {
    Bold,
    ButtonsContainer,
    Global,
    InputContainer,
    Layout,
    LayoutItem,
    LayoutItemBody,
    LayoutItemHeader,
} from "./layouts"
import { getUserAgentId } from "./getUserAgentId"
import { fetchApi } from "./fetchApi"

const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const [id, setId] = useState("")
    const [password, setPassword] = useState("")

    const formRef = useRef<HTMLFormElement>(null)
    const idRef = useRef<HTMLInputElement & HTMLTextAreaElement>(null)
    const passwordRef = useRef<HTMLInputElement & HTMLTextAreaElement>(null)

    useEffect(() => {
        idRef.current?.focus()
    }, [])

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
            state_id: stateId ?? "",
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

        location.replace(res.location)
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
            <Layout>
                <LayoutItem>
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
                                    fullWidth
                                    onClick={onSubmitPassword}
                                    type="submit"
                                    disabled={sendingPassword}
                                >
                                    ログイン
                                </Button>
                                <Button
                                    variant="Primary"
                                    fullWidth
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
            <Global />
        </Themed>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
