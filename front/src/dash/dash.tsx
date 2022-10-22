import React, { useEffect, useState } from 'react'
import { createRoot } from 'react-dom/client'
import { Themed } from '../../theme'
import { useApi } from './useApi'
import { useSideMenu } from './useSideMenu'

const App = () => {
    const [ token, setToken ] = useState('')

    useEffect(() => {
        console.log('effect')
        if (!location.hash && !token) {
            location.replace('/dash/signin')
        } else {
            if (!token) {
                setToken(location.hash.slice(1))
                location.hash = ''
            }
        }
    }, [location.hash])

    const [relyingParties, setRelyingParties] = useState<any[]>([])
    const [ reloadRpList ] = useApi(token, '/dash/rp/list', { token }, (json) => {
        if (json.values) {
            setRelyingParties(json.values)
        }
    })

    const [Menu, currentPage] = useSideMenu()

    return <Themed>
        { currentPage }
        <Menu />
        <ul>{
            relyingParties.map(p => {
                return <li key={ p.client_id }>{ JSON.stringify(p) }</li>
            })
        }</ul>
        <button onClick={ () => { reloadRpList() } }>reloadRpList</button>
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
