import React, { useEffect, useState } from 'react'
import { createRoot } from 'react-dom/client'

import styled from 'styled-components'

import { Button } from '@charcoal-ui/react'
import { theme, Themed } from './theme'
import { useContinueForm } from './useContinueForm'
import { useQueryParams } from './useQueryParams'

const Container = styled.div`
    display: grid;
    gap: ${ ({ theme }) => theme.spacing[24] }px;

    max-width: 600px;

    ${theme(o => [
        o.margin.vertical(24),
        o.margin.horizontal('auto'),
        o.padding.horizontal(8),
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

    const onSubmit = async () => {
        next()
    }

    const chooseOtherAccount = () => {
        const continueUrl = encodeURIComponent(`/signin?sid=${stateId}&cid=${encodeURIComponent(relyingPartyId)}`)
        location.replace(`/signout?continue=${continueUrl}`)
    }

    return <Themed>
        {
            !hidden && <Container>
                <div>{id} さん</div>
                <Button variant='Primary' fixed onClick={ onSubmit }>続ける</Button>
                <Button fixed onClick={ chooseOtherAccount }>アカウントを切り替える</Button>
            </Container>
        }
        <ContinueForm />
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
