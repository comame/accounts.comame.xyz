import { Button } from '@charcoal-ui/react'
import React, { useEffect, useState } from 'react'
import { Modal, ModalBody, ModalHeader } from './modal'
import { useApi } from './useApi'

export function RelyingParty({ token }: { token: string }) {
    const [relyingParties, setRelyingParties] = useState<any[]>([])
    const [ listRpApi ] = useApi(token, '/dash/rp/list', (json) => {
        if (json.values) {
            setRelyingParties(json.values)
        }
    })
    useEffect(() => {
        listRpApi()
    }, [ token ])

    const [ createRelyingPartyApi ] = useApi(token, '/dash/rp/create', () => {
        listRpApi()
    })
    const isModalOpen = useState(true)

    return <>
        <div>
            <Modal open={isModalOpen} isDissmissable={false}>
                <ModalHeader>Hello</ModalHeader>
                <ModalBody>Modal</ModalBody>
            </Modal>
            <div className='p-8 inline-block'><Button size='S' variant='Navigation' onClick={ () => listRpApi() }>RELOAD</Button></div>
            <div className='p-8 inline-block mb-24'><Button size='S' variant='Primary'>CREATE</Button></div>
            {
                relyingParties.sort((a, b) => a.client_id < b.client_id ? -1 : 1).map(rp =>
                    <div key={rp.client_id} className='p-8 mb-16 bg-surface3'>
                        <h2 className='font-bold text-base'>{ rp.client_id }</h2>
                        <div>
                            <div className='inline-block p-8 pl-0'><Button size='S' variant='Navigation'>EDIT</Button></div>
                            <div className='inline-block p-8 pl-0'><Button size='S' variant='Overlay'>DELETE</Button></div>
                            <h3 className='font-bold'>Redirect URIs</h3>
                            <ul>{
                                rp.redirect_uris.map((uri: string) => <li key={uri}>{uri}</li>)
                            }</ul>
                        </div>
                    </div>
                )
            }
        </div>
    </>
}
