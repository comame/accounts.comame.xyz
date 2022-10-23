import { Button, TextField } from "@charcoal-ui/react"
import React, { useEffect, useState } from "react"
import { Modal, ModalBody, ModalHeader } from "./modal"
import { useApi } from "./useApi"

export function RelyingParty({ token }: { token: string }) {
    const [relyingParties, setRelyingParties] = useState<any[]>([])
    const [listRpApi] = useApi(token, "/dash/rp/list", (json) => {
        if (json.values) {
            setRelyingParties(json.values)
        }
    })
    useEffect(() => {
        listRpApi()
    }, [token])

    const [createRelyingPartyApi] = useApi(token, "/dash/rp/create", () => {
        listRpApi()
    })
    const createModalOpen = useState(false)

    return (
        <>
            <div>
                <CreateRPModal
                    open={createModalOpen}
                    createApi={createRelyingPartyApi}
                ></CreateRPModal>
                <div className="p-8 inline-block">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => listRpApi()}
                    >
                        RELOAD
                    </Button>
                </div>
                <div className="p-8 inline-block mb-24">
                    <Button
                        size="S"
                        variant="Primary"
                        onClick={() => createModalOpen[1](true)}
                    >
                        CREATE
                    </Button>
                </div>
                {relyingParties
                    .sort((a, b) => (a.client_id < b.client_id ? -1 : 1))
                    .map((rp) => (
                        <RelyingPartyListItem
                            key={rp.client_id}
                            rp={rp}
                            token={token}
                            updateView={listRpApi}
                        />
                    ))}
            </div>
        </>
    )
}

const RelyingPartyListItem = ({
    rp,
    updateView,
    token,
}: {
    rp: any
    updateView: () => void
    token: string
}) => {
    const deleteModalOpen = useState(false)

    const [deleteRelyingPartyApi] = useApi(token, "/dash/rp/delete", () => {
        updateView()
    })

    const editModalOpen = useState(false)

    return (
        <div key={rp.client_id} className="p-8 mb-16 bg-surface3">
            <DeleteRPModal
                open={deleteModalOpen}
                deleteApi={deleteRelyingPartyApi}
                clientId={rp.client_id}
            />
            <EditModal
                open={editModalOpen}
                rp={rp}
                updateView={updateView}
                token={token}
            />
            <h2 className="font-bold text-base">{rp.client_id}</h2>
            <div>
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => editModalOpen[1](true)}
                    >
                        EDIT
                    </Button>
                </div>
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Overlay"
                        onClick={() => deleteModalOpen[1](true)}
                    >
                        DELETE
                    </Button>
                </div>
                <h3 className="font-bold">Redirect URIs</h3>
                <ul>
                    {rp.redirect_uris.map((uri: string) => (
                        <li key={uri}>{uri}</li>
                    ))}
                </ul>
            </div>
        </div>
    )
}

type createRPModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    createApi: (body: any) => void
}
const CreateRPModal = ({ open, createApi }: createRPModalProps) => {
    const [id, setId] = useState("")
    const onSubmit = () => {
        if (id) {
            createApi({ client_id: id })
            open[1](false)
        }
    }
    const [disabled, setDisabled] = useState(true)
    const onChange = (v: string) => {
        setId(v)
        if (v) {
            setDisabled(false)
        } else {
            setDisabled(true)
        }
    }

    return (
        <Modal open={open} isDissmissable={false}>
            <ModalHeader>Relying Party の作成</ModalHeader>
            <ModalBody>
                <TextField
                    label="client_id"
                    showLabel
                    required
                    className="mb-24"
                    onChange={onChange}
                ></TextField>
                <Button
                    variant="Primary"
                    fixed
                    onClick={onSubmit}
                    disabled={disabled}
                >
                    作成する
                </Button>
            </ModalBody>
        </Modal>
    )
}

type deleteRPModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    deleteApi: (body: any) => void
    clientId: string
}
const DeleteRPModal = ({ open, deleteApi, clientId }: deleteRPModalProps) => {
    const [disabled, setDisabled] = useState(true)
    useEffect(() => {
        if (open[0]) {
            setDisabled(true)
            setTimeout(() => {
                setDisabled(false)
            }, 2000)
        }
    }, [open[0]])

    const onSubmit = () => {
        deleteApi({ client_id: clientId })
        open[1](false)
    }

    return (
        <Modal open={open}>
            <ModalHeader>Relying Party の削除</ModalHeader>
            <ModalBody>
                <div className="mb-24">
                    <span className="font-bold">{clientId}</span>{" "}
                    を削除しますか？
                </div>
                <Button
                    variant="Danger"
                    fixed
                    disabled={disabled}
                    onClick={onSubmit}
                >
                    削除する
                </Button>
            </ModalBody>
        </Modal>
    )
}

type editModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    rp: any
    updateView: () => void
    token: string
}
const EditModal = ({ open, rp, updateView, token }: editModalProps) => {
    const [uris, setUris] = useState<string[]>([...rp.redirect_uris])
    useEffect(() => {
        setUris([...rp.redirect_uris])
    }, [open[0]])
    const onChange = (v: string) => {
        setUris(v.split("\n"))
    }
    const onSubmit = () => {
        const promises = [
            ...rp.redirect_uris.map((uri: string) =>
                deleteUri({ client_id: rp.client_id, redirect_uri: uri })
            ),
            ...uris
                .filter((v) => v.trim() !== "")
                .map((uri: string) =>
                    addUri({ client_id: rp.client_id, redirect_uri: uri })
                ),
        ]
        Promise.allSettled(promises).then(() => {
            updateView()
            open[1](false)
        })
    }

    const [deleteUri] = useApi(token, "/dash/rp/redirect_uri/remove", () => {})
    const [addUri] = useApi(token, "/dash/rp/redirect_uri/add", () => {})

    return (
        <Modal open={open} isDissmissable={false}>
            <ModalHeader>Redirect URI の編集</ModalHeader>
            <ModalBody>
                <TextField
                    multiline
                    label="redirect_uris"
                    placeholder="redirect_uris"
                    value={uris.join("\n")}
                    onChange={onChange}
                    className="mb-24"
                ></TextField>
                <Button variant="Primary" fixed onClick={onSubmit}>
                    確定する
                </Button>
            </ModalBody>
        </Modal>
    )
}
