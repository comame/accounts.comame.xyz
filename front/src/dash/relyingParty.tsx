import { Button, TextField, SelectGroup, Select } from "@charcoal-ui/react"
import React, { Suspense, useEffect, useRef, useState } from "react"
import { diffArray } from "../lib"
import { Modal, ModalBody, ModalHeader } from "./modal"
import { relyingParty } from "./types"
import { useSuspendApi, fetchApi } from "./useApi"
import { useToken } from "./useToken"

export function RelyingParty() {
    const { data: relyingPartiesResponse, mutate: updateView } = useSuspendApi(
        useToken(),
        "/dash/rp/list",
        {}
    )
    const relyingParties = relyingPartiesResponse.values

    useEffect(() => {
        updateView()
    }, [])

    const createModalOpen = useState(false)

    return (
        <>
            <div>
                <CreateRPModal
                    open={createModalOpen}
                    updateView={updateView}
                ></CreateRPModal>
                <div className="p-8 inline-block">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => updateView()}
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
                            updateView={updateView}
                        />
                    ))}
            </div>
        </>
    )
}

const RelyingPartyListItem = ({
    rp,
    updateView,
}: {
    rp: relyingParty
    updateView: () => void
}) => {
    const deleteModalOpen = useState(false)
    const editModalOpen = useState(false)
    const updateSecretModalOpen = useState(false)
    const bindingModalOpen = useState(false)
    const federatedBindingModalOpen = useState(false)

    return (
        <div key={rp.client_id} className="p-8 mb-16 bg-surface3">
            <DeleteRPModal
                open={deleteModalOpen}
                clientId={rp.client_id}
                updateView={updateView}
            />
            <EditModal open={editModalOpen} rp={rp} updateView={updateView} />
            <NewSecretModal
                open={updateSecretModalOpen}
                client_id={rp.client_id}
            />
            <BindingModal open={bindingModalOpen} clientId={rp.client_id} />
            <FederatedBindingModal
                key={rp.client_id}
                open={federatedBindingModalOpen}
                clientId={rp.client_id}
            />
            <h2 className="font-bold text-base">{rp.client_id}</h2>
            <div>
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => editModalOpen[1](true)}
                    >
                        redirect_uris
                    </Button>
                </div>
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => bindingModalOpen[1](true)}
                    >
                        bindings
                    </Button>
                </div>
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => federatedBindingModalOpen[1](true)}
                    >
                        Federated User Binding
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
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Overlay"
                        onClick={() => updateSecretModalOpen[1](true)}
                    >
                        client_secret
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
    updateView: () => void
}
const CreateRPModal = ({ open, updateView }: createRPModalProps) => {
    const [id, setId] = useState("")
    const onSubmit = () => {
        if (id) {
            fetchApi(useToken(), "/dash/rp/create", { client_id: id }).then(
                (res) => {
                    setClientSecret(res.client_secret)
                }
            )
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

    const [clientSecret, setClientSecret] = useState("")

    const onCloseClick = () => {
        setClientSecret("")
        updateView()
        open[1](false)
    }

    return (
        <Modal open={open} isDissmissable={false} onClose={onCloseClick}>
            <ModalHeader>Relying Party ?????????</ModalHeader>
            <ModalBody>
                {!clientSecret && (
                    <>
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
                            ????????????
                        </Button>
                    </>
                )}
                {clientSecret && (
                    <>
                        <TextField
                            label="client_secret"
                            showLabel
                            className="mb-24"
                            value={clientSecret}
                        ></TextField>
                        <Button variant="Primary" fixed onClick={onCloseClick}>
                            ?????????
                        </Button>
                    </>
                )}
            </ModalBody>
        </Modal>
    )
}

type deleteRPModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    clientId: string
    updateView: () => void
}
const DeleteRPModal = ({ open, clientId, updateView }: deleteRPModalProps) => {
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
        fetchApi(useToken(), "/dash/rp/delete", { client_id: clientId }).then(
            () => {
                updateView()
                open[1](false)
            }
        )
    }

    return (
        <Modal open={open}>
            <ModalHeader>Relying Party ?????????</ModalHeader>
            <ModalBody>
                <div className="mb-24">
                    <span className="font-bold">{clientId}</span>{" "}
                    ????????????????????????
                </div>
                <Button
                    variant="Danger"
                    fixed
                    disabled={disabled}
                    onClick={onSubmit}
                >
                    ????????????
                </Button>
            </ModalBody>
        </Modal>
    )
}

type bindingModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    clientId: string
}
const BindingModal = ({ open, clientId }: bindingModalProps) => {
    return (
        <Modal open={open}>
            <ModalHeader>UserBinding</ModalHeader>
            <ModalBody>
                <Suspense fallback={<>Loading</>}>
                    <div className="p-8">
                        <Bindings clientId={clientId} open={open[0]} />
                    </div>
                </Suspense>
            </ModalBody>
        </Modal>
    )
}

const Bindings = ({ clientId, open }: { clientId: string; open: boolean }) => {
    const { data: bindings, mutate: mutateA } = useSuspendApi(
        useToken(),
        "/dash/rp/binding/list",
        {
            client_id: clientId,
        },
        `/dash/rp/binding/list/${clientId}`
    )
    const { data: users, mutate: mutateB } = useSuspendApi(
        useToken(),
        "/dash/user/list",
        {}
    )

    const [selected, setSelected] = useState(
        bindings.values.map((v) => v.user_id)
    )

    const [hasUpdate, setHasUpdate] = useState(false)
    useEffect(() => {
        if (open) {
            mutateA()
            mutateB()
            setHasUpdate(true)
        }
    }, [open])

    useEffect(() => {
        if (hasUpdate) {
            setSelected(bindings.values.map((v) => v.user_id))
            setHasUpdate(false)
        }
    }, [hasUpdate])

    const onClick = async () => {
        setDisabled(true)
        const diff = diffArray(
            bindings.values.map((v) => v.user_id),
            selected
        )
        for (const addUser of diff.add) {
            await fetchApi(useToken(), "/dash/rp/binding/add", {
                client_id: clientId,
                user_id: addUser,
            })
        }
        for (const delUser of diff.del) {
            await fetchApi(useToken(), "/dash/rp/binding/remove", {
                client_id: clientId,
                user_id: delUser,
            })
        }
        setDisabled(false)
    }

    const [disabled, setDisabled] = useState(false)

    return (
        <>
            <SelectGroup
                name="user-binding"
                ariaLabel="user-binding"
                selected={selected}
                onChange={setSelected}
                disabled={disabled}
                className="mb-16"
            >
                {users.values.map((user) => (
                    <div key={user.user_id} className="mb-4">
                        <Select value={user.user_id}>{user.user_id}</Select>
                    </div>
                ))}
            </SelectGroup>
            <Button onClick={onClick} fixed disabled={disabled}>
                ????????????
            </Button>
        </>
    )
}

type federatedBindingModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    clientId: string
}
const FederatedBindingModal = ({
    open,
    clientId,
}: federatedBindingModalProps) => {
    return (
        <Modal open={open}>
            <ModalHeader>UserBinding</ModalHeader>
            <ModalBody>
                <Suspense fallback={<>Loading</>}>
                    <div className="p-8">
                        <FederatedBindings clientId={clientId} open={open[0]} />
                    </div>
                </Suspense>
            </ModalBody>
        </Modal>
    )
}

const FederatedBindings = ({
    clientId,
    open,
}: {
    clientId: string
    open: boolean
}) => {
    const { data: bindings, mutate: mutateA } = useSuspendApi(
        useToken(),
        "/dash/rp/federated_user_binding/list",
        {
            client_id: clientId,
        },
        `/dash/rp/federated_user_binding/list/${clientId}`
    )
    const issuers = ["google"]

    const [selected, setSelected] = useState(
        bindings.values.map((v) => v.issuer)
    )

    const [hasUpdate, setHasUpdate] = useState(false)
    useEffect(() => {
        if (open) {
            mutateA()
            setHasUpdate(true)
        }
    }, [open])

    useEffect(() => {
        if (hasUpdate) {
            setSelected(bindings.values.map((v) => v.issuer))
            setHasUpdate(false)
        }
    }, [hasUpdate])

    const onClick = async () => {
        setDisabled(true)
        const diff = diffArray(
            bindings.values.map((v) => v.issuer),
            selected
        )
        for (const addUser of diff.add) {
            await fetchApi(useToken(), "/dash/rp/federated_user_binding/add", {
                client_id: clientId,
                issuer: addUser,
            })
        }
        for (const delUser of diff.del) {
            await fetchApi(
                useToken(),
                "/dash/rp/federated_user_binding/remove",
                {
                    client_id: clientId,
                    issuer: delUser,
                }
            )
        }
        setDisabled(false)
    }

    const [disabled, setDisabled] = useState(false)

    return (
        <>
            <SelectGroup
                name="user-binding"
                ariaLabel="user-binding"
                selected={selected}
                onChange={setSelected}
                disabled={disabled}
                className="mb-16"
            >
                {issuers.map((issuer) => (
                    <div key={issuer} className="mb-4">
                        <Select value={issuer}>{issuer}</Select>
                    </div>
                ))}
            </SelectGroup>
            <Button onClick={onClick} fixed disabled={disabled}>
                ????????????
            </Button>
        </>
    )
}

type newSecretModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    client_id: string
}
const NewSecretModal = ({ open, client_id }: newSecretModalProps) => {
    const [secret, setSecret] = useState("")
    const onClick = async () => {
        const res = await fetchApi(useToken(), "/dash/rp/update_secret", {
            client_id,
        })
        setSecret(res.client_secret)
    }
    useEffect(() => {
        if (!open[0]) {
            setSecret("")
        }
    }, [open[0]])
    return (
        <Modal open={open} isDissmissable={false}>
            <ModalHeader>client_secret ?????????</ModalHeader>
            <ModalBody>
                <div className="mb-8">
                    <Button variant="Danger" fixed onClick={onClick}>
                        ????????????
                    </Button>
                </div>
                <TextField label="client_secret" showLabel value={secret} />
            </ModalBody>
        </Modal>
    )
}

type editModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    rp: any
    updateView: () => void
}
const EditModal = ({ open, rp, updateView }: editModalProps) => {
    const [uris, setUris] = useState<string[]>([...rp.redirect_uris])
    useEffect(() => {
        setUris([...rp.redirect_uris])
    }, [open[0]])
    const onChange = (v: string) => {
        setUris(v.split("\n"))
    }
    const onSubmit = () => {
        const deletes = [
            ...rp.redirect_uris.map((uri: string) =>
                fetchApi(useToken(), "/dash/rp/redirect_uri/remove", {
                    client_id: rp.client_id,
                    redirect_uri: uri,
                })
            ),
        ]
        const adds = [
            ...uris
                .filter((v) => v.trim() !== "")
                .map((uri: string) =>
                    fetchApi(useToken(), "/dash/rp/redirect_uri/add", {
                        client_id: rp.client_id,
                        redirect_uri: uri,
                    })
                ),
        ]

        Promise.allSettled(deletes)
            .then(() => Promise.allSettled(adds))
            .then(() => {
                updateView()
                open[1](false)
            })
    }

    return (
        <Modal open={open} isDissmissable={false}>
            <ModalHeader>Redirect URI ?????????</ModalHeader>
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
                    ????????????
                </Button>
            </ModalBody>
        </Modal>
    )
}
