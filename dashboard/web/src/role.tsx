import React, { useEffect, useState } from "react"
import { fetchApi, mutateAll, useSuspendApi } from "./useApi"
import { useToken } from "./useToken"
import { TextField, Button } from "@charcoal-ui/react"
import { Modal, ModalHeader, ModalBody } from "./modal"

export default function Role() {
    const rolesRes = useSuspendApi(useToken(), "/role/list", {})

    const updateView = () => {
        mutateAll()
    }

    const createModalOpen = useState(false)

    return (
        <>
            <div>
                <div className="p-8 inline-block">
                    <Button size="S" variant="Navigation" onClick={updateView}>
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
                    <CreateRoleModal
                        open={createModalOpen}
                        updateView={updateView}
                    />
                </div>
                {rolesRes.data.values.sort().map((role) => (
                    <RoleItem name={role} updateView={updateView} />
                ))}
            </div>
        </>
    )
}

type roleItemProps = {
    name: string
    updateView: () => void
}
function RoleItem({ name, updateView }: roleItemProps) {
    const deleteModalOpen = useState(false)

    return (
        <div className="p-8 mb-16 bg-surface3">
            <h2 className="font-bold text-base mb-8">{name}</h2>
            <div className="mb-8">
                <div className="inline-block p-8 pl-0">
                    <Button
                        size="S"
                        variant="Overlay"
                        onClick={() => deleteModalOpen[1](true)}
                    >
                        DELETE
                    </Button>
                    <DeleteRoleModal
                        updateView={updateView}
                        name={name}
                        open={deleteModalOpen}
                    />
                </div>
            </div>
        </div>
    )
}

type createRoleModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    updateView: () => void
}
const CreateRoleModal = ({ open, updateView }: createRoleModalProps) => {
    const [id, setId] = useState("")
    const onSubmit = () => {
        if (id) {
            fetchApi(useToken(), "/role/create", { name: id }).then(() => {
                open[1](false)
                updateView()
            })
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
            <ModalHeader>ロールの作成</ModalHeader>
            <ModalBody>
                <TextField
                    label="role"
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

type deleteRoleModalProps = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    name: string
    updateView: () => void
}
const DeleteRoleModal = ({ open, name, updateView }: deleteRoleModalProps) => {
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
        fetchApi(useToken(), "/role/delete", { name }).then(() => {
            updateView()
            open[1](false)
        })
    }

    return (
        <Modal open={open}>
            <ModalHeader>ロールの削除</ModalHeader>
            <ModalBody>
                <div className="mb-24">
                    <span className="font-bold">{name}</span> を削除しますか？
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
