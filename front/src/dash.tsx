import React, { useEffect, useState } from 'react'
import { createRoot } from 'react-dom/client'

const useApiEffect = (token: string, effect: () => void, deps: any[]) => {
    useEffect(() => {
        if (token) {
            effect()
        }
    }, [token, ...deps])
}

const App = () => {
    const [ token, setToken ] = useState('')

    useEffect(() => {
        if (!location.hash && !token) {
            location.replace('/dash/signin')
        } else {
            setToken(location.hash.slice(1))
            location.hash = ''
        }
    }, [])

    const [relyingParties, setRelyingParties] = useState<any[]>([])
    useApiEffect(token, () => {
        console.log('api')
        fetch('/dash/rp/list', {
            method: 'POST',
            body: JSON.stringify({ token })
        }).then(res => res.json()).then(json => {
            if (json.values) {
                setRelyingParties(json.values)
            }
        })
    }, [])

    return <div><ul>{
        relyingParties.map(p => {
            return <li key={ p.client_id }>{ JSON.stringify(p) }</li>
        })
    }</ul></div>
}

createRoot(document.getElementById('app')!).render(<App />)
