import { Layout, LayoutItem, LayoutItemBody } from '@charcoal-ui/react-sandbox'
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

    const toNormalRepresentation = (msg: string) => {
        return msg.split('-').map(word => {
            return word[0].toUpperCase() + word.slice(1)
        }).join(' ')
    }

    const [Menu, currentPage] = useSideMenu()
    const Header = <div>{ toNormalRepresentation(currentPage) }</div>

    return <Themed>
        <Layout menu={ Menu } header={ Header } wide>
            <LayoutItem span={ 3 }>
                <LayoutItemBody>
                    <ul>{
                        relyingParties.map(p => {
                            return <li key={ p.client_id }>{ JSON.stringify(p) }</li>
                        })
                    }</ul>
                    <button onClick={ () => { reloadRpList() } }>reloadRpList</button>
                </LayoutItemBody>
            </LayoutItem>
        </Layout>
    </Themed>
}

createRoot(document.getElementById('app')!).render(<App />)
