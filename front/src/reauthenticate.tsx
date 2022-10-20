import React, { useEffect, useState } from 'react'
import { createRoot } from 'react-dom/client'

import styled, { createGlobalStyle } from 'styled-components'

import { Button, TextField } from '@charcoal-ui/react'
import { theme, Themed } from './theme'
import { useContinueForm } from './useContinueForm'
import { useQueryParams } from './useQueryParams'

const Global = createGlobalStyle`
    html {
        ${theme(o => [
            o.bg.surface3,
        ])}
        font-family: sans-serif;
    }
`

const TextContainer = styled.div`
    line-height: 2;

    ${theme(o => [
        o.margin.top(24),
        o.font.text1,
    ])}
`

const Bold = styled.span`
    font-weight: bold;
`

const FormContainer = styled.form`
    max-width: 500px;

    ${theme(o => [
        o.bg.background1,
        o.margin.horizontal('auto'),
        o.margin.top(24),
        o.padding.top(16),
        o.padding.bottom(40),
        o.padding.horizontal(24),
        o.borderRadius(24),
    ])}
`

const InputContainer = styled.div`
    display: grid;
    gap: ${ ({ theme }) => theme.spacing[24] }px;

    ${theme(o => [
        o.margin.top(40),
    ])}
`

const ButtonsContainer = styled.div`
    display: grid;
    gap: ${ ({ theme }) => theme.spacing[24] }px;

    ${theme(o => [
        o.margin.top(64),
    ])}
`

const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const [loginType, setLoginType] = useState('')
    const [hidden, setHidden] = useState(true)

    const [ ContinueForm, _ref, next ] = useContinueForm(
        csrfToken,
        loginType,
        stateId ?? undefined,
        relyingPartyId
    )

    useEffect(() => {
        fetch('/api/signin-session', {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                csrf_token: csrfToken
            })
        }).then(res => res.json()).then(json => {
            if (json['user_id']) {
                setId(json['user_id'])
                setHidden(false)
            } else {
                location.replace(`/signin?sid=${stateId}&cid=${encodeURIComponent(relyingPartyId)}`)
            }
        })
    }, [])

    const [ id, setId ] = useState('')
    const [ password, setPassword ] = useState('')

    const onSubmitPassword = async (e: React.FormEvent) => {
        e.preventDefault()

        const body = JSON.stringify({
            user_id: id,
            password,
            csrf_token: csrfToken
        })
        const res = await fetch('/api/signin-password', {
            method: 'POST',
            body,
            headers: {
                'Content-Type': 'application/json'
            }
        })
        if (res.status !== 200) {
            return
        }

        setLoginType('password')
        next()
    }

    const chooseOtherAccount = (e: React.FormEvent) => {
        e.preventDefault()
        const continueUrl = encodeURIComponent(`${location.origin}/signin?sid=${stateId}&cid=${encodeURIComponent(relyingPartyId)}`)
        location.replace(`/signout?continue=${continueUrl}`)
    }

    return <Themed>
        {
            !hidden && <FormContainer>
                <TextContainer>
                    <div><Bold>{ relyingPartyId }</Bold> にログインする</div>
                </TextContainer>
                <TextContainer>
                    <div>{ id } さん</div>
                    <div>続けるには、パスワードを入力してください</div>
                </TextContainer>
                <InputContainer>
                    <input type='hidden' onChange={ e => setId(e.target.value) }></input>
                    <TextField label='パスワード' placeholder='パスワード' type='password' required onChange={ e => setPassword(e) }></TextField>
                </InputContainer>
                <ButtonsContainer>
                    <Button variant='Primary' fixed onClick={ onSubmitPassword } autoFocus>続ける</Button>
                    <Button fixed onClick={ chooseOtherAccount }>アカウントを切り替える</Button>
                </ButtonsContainer>
            </FormContainer>
        }
        <ContinueForm />
        <Global />
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
