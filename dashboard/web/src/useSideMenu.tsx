import { LeftMenu } from "@charcoal-ui/react-sandbox"
import React, { useEffect, useState } from "react"

type pages = "relying-party" | "user" | "role"
const pages: pages[] = ["relying-party", "user", "role"]
function isPage(arg: any): arg is pages {
    return pages.includes(arg)
}

export const useSideMenu: () => [Menu: JSX.Element, page: pages] = () => {
    const [currentPage, setCurrentPage] = useState<pages>("relying-party")

    useEffect(() => {
        const page = location.hash.slice(1)
        if (isPage(page)) {
            setCurrentPage(page)
        }
    }, [location.hash])

    useEffect(() => {
        window.addEventListener("hashchange", () => {
            const page = location.hash.slice(1)
            if (isPage(page)) {
                setCurrentPage(page)
            }
        })
    }, [])

    const links = pages.map((page) => ({
        id: page,
        text: page,
        to: "#" + page,
    }))

    const element = <LeftMenu links={links} active={currentPage}></LeftMenu>

    return [element, currentPage]
}
