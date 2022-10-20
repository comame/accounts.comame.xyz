import React, { useEffect, useMemo, useRef, useState } from 'react'
import { createRoot } from 'react-dom/client'

import styled from 'styled-components'

import { Button, TextField } from '@charcoal-ui/react'
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

    const onSubmitPassword = async () => {
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
            !hidden && <Container>
                <TextField showLabel label='ID' required onChange={ e => setId(e) }></TextField>
                <TextField showLabel label='パスワード' type='password' required onChange={ e => setPassword(e) }></TextField>
                <Button variant='Primary' fixed onClick={ onSubmitPassword }>ログイン</Button>
            </Container>
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
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
