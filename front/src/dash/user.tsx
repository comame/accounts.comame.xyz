import { Button, TextField } from "@charcoal-ui/react"
import React, { Suspense, useEffect, useState } from "react"
import { Modal, ModalBody, ModalHeader } from "./modal"
import { fetchApi, useSuspendApi } from "./useApi"
import { useToken } from "./useToken"

type user = {
    user_id: string
    has_password: boolean
}

export function User() {
    const { data: usersResponse, mutate } = useSuspendApi(
        useToken(),
        "/dash/user/list",
        {}
    )
    const users = usersResponse.values

    const createModalOpen = useState(false)

    useEffect(() => {
        mutate()
    }, [])

    return (
        <>
            <div>
                <div className="p-8 inline-block">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => mutate()}
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
                    <CreateUserModal
                        open={createModalOpen}
                        updateView={mutate}
                    />
                </div>
                {users
                    .sort((a, b) => (a.user_id < b.user_id ? -1 : 1))
                    .map((user) => (
                        <UserListItem
                            key={user.user_id}
                            user={user}
                            updateView={mutate}
                        />
                    ))}
            </div>
        </>
    )
}

const UserListItem = ({
    user,
    updateView,
}: {
    user: user
    updateView: () => void
}) => {
    const deleteModalOpen = useState(false)

    const passwordEditOpen = useState(false)

    const logModalOpen = useState(false)

    return (
        <div key={user.user_id} className="p-8 mb-16 bg-surface3">
            <h2 className="font-bold text-base mb-8">{user.user_id}</h2>
            <div className="mb-8">
                ??????????????? {user.has_password ? "????????????" : "?????????"}
            </div>
            <div className="mb-8">
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => passwordEditOpen[1](true)}
                    >
                        PASSWORD
                    </Button>
                    <SetPasswordModal
                        updateView={updateView}
                        open={passwordEditOpen}
                        userId={user.user_id}
                    />
                </div>
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Overlay"
                        onClick={() => deleteModalOpen[1](true)}
                    >
                        DELETE
                    </Button>
                    <DeleteUserModal
                        open={deleteModalOpen}
                        userId={user.user_id}
                        updateView={updateView}
                    />
                </div>
            </div>
            <div>
                <Button
                    size="S"
                    variant="Navigation"
                    onClick={() => logModalOpen[1](true)}
                >
                    ??????????????????
                </Button>
                <LogModal userId={user.user_id} open={logModalOpen} />
            </div>
        </div>
    )
}

type logModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    userId: string
}
const LogModal = ({ open, userId }: logModalProps) => {
    return (
        <Modal open={open}>
            <ModalHeader>??????????????????</ModalHeader>
            <ModalBody>
                <Suspense fallback={<>Loading</>}>
                    <Logs userId={userId} />
                </Suspense>
            </ModalBody>
        </Modal>
    )
}

const Logs = ({ userId }: { userId: string }) => {
    const { data, mutate } = useSuspendApi(
        useToken(),
        "/dash/user/authentication/list",
        { user_id: userId },
        "/dash/user/authentication/list/" + userId
    )
    useEffect(() => {
        mutate()
    }, [])
    return (
        <ul>
            {data.values.map((log) => (
                <li key={log.iat}>
                    {new Date(log.iat * 1000).toLocaleString()}{" "}
                    {`${log.remote_addr}`}: {log.aud}
                </li>
            ))}
        </ul>
    )
}

type createUserModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    updateView: () => void
}
const CreateUserModal = ({ open, updateView }: createUserModalProps) => {
    const [id, setId] = useState("")
    const onSubmit = () => {
        if (id) {
            fetchApi(useToken(), "/dash/user/create", { user_id: id }).then(
                () => {
                    open[1](false)
                    updateView()
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

    return (
        <Modal open={open} isDissmissable={false}>
            <ModalHeader>?????????????????????</ModalHeader>
            <ModalBody>
                <TextField
                    label="user_id"
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
            </ModalBody>
        </Modal>
    )
}

type deleteUserModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    userId: string
    updateView: () => void
}
const DeleteUserModal = ({
    open,
    userId,
    updateView,
}: deleteUserModalProps) => {
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
        fetchApi(useToken(), "/dash/user/delete", { user_id: userId }).then(
            () => {
                updateView()
                open[1](false)
            }
        )
    }

    return (
        <Modal open={open}>
            <ModalHeader>?????????????????????</ModalHeader>
            <ModalBody>
                <div className="mb-24">
                    <span className="font-bold">{userId}</span> ????????????????????????
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

type setPasswordModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    userId: string
    updateView: () => void
}
const SetPasswordModal = ({
    userId,
    open,
    updateView,
}: setPasswordModalProps) => {
    const [password, setPassword] = useState("")

    useEffect(() => {
        if (open[0]) {
            setPassword("")
        }
    }, [open[0]])

    const onSubmit = () => {
        if (password == "") {
            fetchApi(useToken(), "/dash/user/password/remove", {
                user_id: userId,
            }).then(() => {
                updateView()
                open[1](false)
            })
        } else {
            fetchApi(useToken(), "/dash/user/password/change", {
                user_id: userId,
                password,
            }).then(() => {
                updateView()
                open[1](false)
            })
        }
    }

    return (
        <Modal open={open}>
            <ModalHeader>????????????????????????</ModalHeader>
            <ModalBody>
                <div className="mb-24">
                    <span className="font-bold">{userId}</span>{" "}
                    ???????????????????????????
                </div>
                <TextField
                    label="???????????????"
                    placeholder="???????????????"
                    className="mb-24"
                    onChange={(v) => setPassword(v)}
                ></TextField>
                <Button variant="Primary" fixed onClick={onSubmit}>
                    ????????????
                </Button>
            </ModalBody>
        </Modal>
    )
}
