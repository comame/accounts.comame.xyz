import React, { useEffect, useRef, useState } from 'react'

export function useContinueForm(
    csrfToken: string|undefined,
    loginType: string|undefined,
    stateId: string|undefined,
    relyingPartyId: string|undefined
): [
    node: React.FC,
    ref: React.RefObject<HTMLFormElement>,
    next: () => void,
] {
    const ref = useRef<HTMLFormElement>(null)

    const [next, setNext] = useState(false)

    // Delays 1 tick
    const onSubmit = () => {
        setNext(true)
    }

    useEffect(() => {
        if (next) {
            console.log(element)
            ref.current?.submit()
        }
    }, [next])

    const element = () => <form
        ref={ ref }
        action='/api/signin-continue'
        method='POST'
        encType='application/x-www-form-urlencoded'
        target="_self"
    >
        <input name='csrf_token' type='hidden' value={ csrfToken }></input>
        <input name='login_type' type='hidden' value={ loginType }></input>
        <input name='state_id' type='hidden' value={ stateId }></input>
        <input name='relying_party_id' type='hidden' value={ relyingPartyId }></input>
    </form>

    return [ element, ref, onSubmit ]
}
