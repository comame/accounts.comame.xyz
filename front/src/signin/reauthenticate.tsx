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
import { fetchApi } from "./fetchApi"

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
        fetchApi("/api/signin-session", {
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
        }).then((res) => {
            if ("error" in res && res.error === "bad_request") {
                throw "bad_request"
            }

            if ("error" in res && res.error === "no_session") {
                location.replace(
                    `/signin?sid=${stateId}&cid=${encodeURIComponent(
                        relyingPartyId
                    )}`
                )
            }

            if ("error" in res) {
                throw "unreachable"
            }

            setId(res.user_id)
            setHidden(false)
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

        const body = {
            user_id: id,
            password,
            csrf_token: csrfToken,
            relying_party_id: relyingPartyId,
            user_agent_id: getUserAgentId(),
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
                                <Bold>{relyingPartyId}</Bold> ???????????????
                            </div>
                        </LayoutItemHeader>
                        <LayoutItemBody>
                            <form ref={formRef}>
                                <TextContainer>
                                    <div>
                                        <Bold>{id}</Bold> ??????
                                    </div>
                                    <div>
                                        ????????????????????????????????????????????????????????????
                                    </div>
                                </TextContainer>
                                <InputContainer>
                                    <TextField
                                        label="???????????????"
                                        placeholder="???????????????"
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
                                                ? "??????????????????????????????????????????"
                                                : isEmpty
                                                ? "??????????????????????????????????????????"
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
                                        ?????????
                                    </Button>
                                    <Button
                                        fixed
                                        onClick={chooseOtherAccount}
                                        disabled={sendingPassword}
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
