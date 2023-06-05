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
import { fetchApi } from "./fetchApi"

const App = () => {
    const { stateId, relyingPartyId, csrfToken, userId } = useQueryParams()

    const [invalidCredential, setInvalidCredential] = useState(false)
    const [sendingPassword, setSendingPassword] = useState(false)
    const [isEmpty, setIsEmpty] = useState(false)

    const formRef = useRef<HTMLFormElement>(null)
    const passwordRef = useRef<HTMLInputElement & HTMLTextAreaElement>(null)

    const [password, setPassword] = useState("")

    useEffect(() => {
        passwordRef.current?.focus()
    }, [])

    const onSubmitPassword = async (e: React.FormEvent) => {
        e.preventDefault()

        const valid = formRef.current?.reportValidity()

        if (!valid || !password) {
            setIsEmpty(true)
            return
        }

        setSendingPassword(true)

        const body = {
            user_id: userId ?? "",
            password,
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
            state_id: stateId ?? "",
        }
        const res = await fetchApi("/api/signin-password", body)

        if ("error" in res) {
            if (res.error === "bad_request") {
                throw "bad_request"
            } else if (res.error === "invalid_credential") {
                setInvalidCredential(true)
                setSendingPassword(false)
                passwordRef.current?.focus()
                return
            }
        } else {
            location.replace(res.location)
        }
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
                                    <Bold>{userId}</Bold> さん
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
            <Global />
        </Themed>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
