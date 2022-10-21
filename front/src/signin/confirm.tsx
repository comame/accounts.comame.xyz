import React, { useEffect, useState } from 'react'
import { createRoot } from 'react-dom/client'

import styled, { createGlobalStyle } from 'styled-components'

import { Button } from '@charcoal-ui/react'
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

const ButtonsContainer = styled.div`
    display: grid;
    gap: ${ ({ theme }) => theme.spacing[24] }px;

    ${theme(o => [
        o.margin.top(40),
    ])}
`


const App = () => {
    const { stateId, relyingPartyId, csrfToken } = useQueryParams()

    const [hidden, setHidden] = useState(true)

    const [ ContinueForm, _ref, next ] = useContinueForm(
        csrfToken,
        'consent',
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

    const onSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
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
                    <div><Bold>{ relyingPartyId }</Bold> にログイン</div>
                </TextContainer>
                <TextContainer>
                    <div><Bold>{id}</Bold> さん</div>
                    <div>続行しますか？</div>
                </TextContainer>
                <ButtonsContainer>
                    <Button variant='Primary' fixed onClick={ onSubmit } autoFocus>続ける</Button>
                    <Button fixed onClick={ chooseOtherAccount }>アカウントを切り替える</Button>
                </ButtonsContainer>
            </FormContainer>
        }
        <ContinueForm />
        <Global />
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
