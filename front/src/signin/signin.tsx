import React, { useEffect, useMemo, useRef, useState } from 'react'
import { createRoot } from 'react-dom/client'

import styled, { createGlobalStyle } from 'styled-components'

import { Button, TextField } from '@charcoal-ui/react'
import { theme, Themed } from '../../theme'
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

    const hash = useMemo(() => location.hash.slice(1), [])
    const maxage = useMemo(() =>
        Number.parseInt(new URL(location.href).searchParams.get('age') ?? '0', 10),
    [])

    const failRef = useRef<HTMLFormElement>(null)

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
        }).then(res => {
            if (res.status !== 200 && hash === 'nointeraction') {
                return Promise.reject('nointeraction-fail')
            } else if (res.status !== 200) {
                return Promise.reject('no-session')
            }
            return res.json()
        }).catch(reason => {
            if (reason === 'nointeraction-fail') {
                console.log(failRef.current)
                failRef.current?.submit()
            } else if (reason === 'no-session') {
                setHidden(false)
            }
        }).then(json => {
            if (!json) {
                return
            }
            if (hash !== 'maxage') {
                setLoginType('session')
                next()
            } else {
                const lastAuth = json['last_auth']
                const now = Math.trunc(Date.now() / 1000)
                if (now <= lastAuth + maxage - 3 /* lag */) {
                    setLoginType('session')
                    next()
                } else {
                    location.replace(`/reauthenticate?sid=${stateId}&cid=${encodeURIComponent(relyingPartyId)}`)
                }
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

    return <Themed>
        {
            !hidden && <FormContainer>
                <TextContainer>
                    <div><Bold>{ relyingPartyId }</Bold> にログイン</div>
                </TextContainer>
                <InputContainer>
                    <TextField showLabel label='ID' required onChange={ e => setId(e) }></TextField>
                    <TextField showLabel label='パスワード' type='password' required onChange={ e => setPassword(e) }></TextField>
                </InputContainer>
                <ButtonsContainer>
                    <Button variant='Primary' fixed onClick={ onSubmitPassword } type='submit'>ログイン</Button>
                </ButtonsContainer>
            </FormContainer>
        }
        <ContinueForm />
        <form
            action='/api/signin-continue-nointeraction-fail'
            method='POST'
            encType='application/x-www-form-urlencoded'
            target='_self'
            ref={failRef}
        >
            <input name='csrf_token' type='hidden' value={ csrfToken }></input>
            <input name='state_id' type='hidden' value={ stateId ?? '' }></input>
        </form>
        <Global />
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
