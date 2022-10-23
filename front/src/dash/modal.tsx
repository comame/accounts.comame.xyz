import React, { ReactNode } from "react"
import { Button } from "@charcoal-ui/react"

import "./Close.svg"

type props = {
    open: [boolean, React.Dispatch<React.SetStateAction<boolean>>]
    children: ReactNode
    isDissmissable?: boolean
    onClose?: () => void
}

export function Modal({
    open,
    children,
    isDissmissable = true,
    onClose,
}: props) {
    const onBackgroundClick = (e: React.MouseEvent) => {
        e.stopPropagation()
        e.preventDefault()
        if (isDissmissable) {
            open[1](false)
        }
    }

    const stopPropagation = (e: React.MouseEvent) => {
        e.stopPropagation()
        e.preventDefault()
    }

    return open[0] ? (
        <>
            <div
                className="modal-bg-w h-screen bg-surface4 z-10 fixed top-0 left-0 flex justify-center items-center"
                onClick={onBackgroundClick}
            >
                <div
                    className="bg-background1 w-col-span-5 p-24 rounded-24 relative"
                    onClick={stopPropagation}
                >
                    {children}
                    <div className="absolute top-16 right-16 z-20">
                        <Button
                            onClick={() => {
                                open[1](false)
                                onClose?.()
                            }}
                        >
                            <svg width={24} height={24}>
                                <use xlinkHref="/front/Close.svg#Close"></use>
                            </svg>
                        </Button>
                    </div>
                </div>
            </div>
        </>
    ) : null
}

type headerProps = {
    children: ReactNode
}
export function ModalHeader({ children }: headerProps) {
    return <div className="font-bold text-center mb-24">{children}</div>
}

type bodyProps = {
    children: ReactNode
}
export function ModalBody({ children }: bodyProps) {
    return <div>{children}</div>
}
