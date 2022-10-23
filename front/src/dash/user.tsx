import { Button, TextField } from "@charcoal-ui/react"
import React, { useEffect, useState } from "react"
import { Modal, ModalBody, ModalHeader } from "./modal"
import { useApi } from "./useApi"

export function User({ token }: { token: string }) {
    const [users, setUsers] = useState<any[]>([])
    const [listUserApi] = useApi(token, "/dash/user/list", (json) => {
        if (json.values) {
            setUsers(json.values)
        }
    })
    useEffect(() => {
        listUserApi()
    }, [token])

    const [createUserApi] = useApi(token, "/dash/user/create", () => {
        listUserApi()
    })
    const createModalOpen = useState(false)

    return (
        <>
            <div>
                <div className="p-8 inline-block">
                    <Button
                        size="S"
                        variant="Navigation"
                        onClick={() => listUserApi()}
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
                        createApi={createUserApi}
                    />
                </div>
                {users
                    .sort((a, b) => (a.user_id < b.user_id ? -1 : 1))
                    .map((user) => (
                        <UserListItem
                            key={user.user_id}
                            user={user}
                            token={token}
                            updateView={listUserApi}
                        />
                    ))}
            </div>
        </>
    )
}

const UserListItem = ({
    user,
    updateView,
    token,
}: {
    user: any
    updateView: () => void
    token: string
}) => {
    const deleteModalOpen = useState(false)
    const [deleteApi] = useApi(token, "/dash/user/delete", () => {
        updateView()
    })

    const passwordEditOpen = useState(false)

    return (
        <div key={user.user_id} className="p-8 mb-16 bg-surface3">
            <h2 className="font-bold text-base mb-16">{user.user_id}</h2>
            <div className="mb-16">
                パスワード {user.has_password ? "設定済み" : "未設定"}
            </div>
            <div>
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
                        token={token}
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
                        deleteApi={deleteApi}
                        userId={user.user_id}
                    />
                </div>
            </div>
        </div>
    )
}

type createUserModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    createApi: (body: any) => void
}
const CreateUserModal = ({ open, createApi }: createUserModalProps) => {
    const [id, setId] = useState("")
    const onSubmit = () => {
        if (id) {
            createApi({ user_id: id })
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
            <ModalHeader>ユーザーの作成</ModalHeader>
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
                    作成する
                </Button>
            </ModalBody>
        </Modal>
    )
}

type deleteUserModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    deleteApi: (body: any) => void
    userId: string
}
const DeleteUserModal = ({ open, deleteApi, userId }: deleteUserModalProps) => {
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
        deleteApi({ user_id: userId })
        open[1](false)
    }

    return (
        <Modal open={open}>
            <ModalHeader>ユーザーの削除</ModalHeader>
            <ModalBody>
                <div className="mb-24">
                    <span className="font-bold">{userId}</span> を削除しますか？
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

type setPasswordModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    token: string
    userId: string
    updateView: () => void
}
const SetPasswordModal = ({
    token,
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

    const [setPasswordApi] = useApi(token, "/dash/user/password/change", () => {
        updateView()
    })
    const [removePasswordApi] = useApi(
        token,
        "/dash/user/password/remove",
        () => {
            updateView()
        }
    )

    const onSubmit = () => {
        if (password == "") {
            removePasswordApi({ user_id: userId })
        } else {
            setPasswordApi({ user_id: userId, password })
        }
        open[1](false)
    }

    return (
        <Modal open={open}>
            <ModalHeader>パスワードの設定</ModalHeader>
            <ModalBody>
                <TextField
                    label="パスワード"
                    placeholder="パスワード"
                    className="mb-24"
                    onChange={(v) => setPassword(v)}
                ></TextField>
                <Button variant="Primary" fixed onClick={onSubmit}>
                    変更する
                </Button>
            </ModalBody>
        </Modal>
    )
}
