import { Layout, LayoutItem, LayoutItemBody } from "@charcoal-ui/react-sandbox"
import React, { useEffect, useState } from "react"
import { createRoot } from "react-dom/client"
import { Themed } from "../theme"
import { RelyingParty } from "./relyingParty"
import { useSideMenu } from "./useSideMenu"

const App = () => {
    const [token, setToken] = useState("")

    useEffect(() => {
        if (!location.hash && !token) {
            location.replace("/dash/signin")
        } else {
            if (!token) {
                setToken(location.hash.slice(1))
                location.hash = ""
            }
        }
    }, [location.hash])

    const toNormalRepresentation = (msg: string) => {
        return msg
            .split("-")
            .map((word) => {
                return word[0].toUpperCase() + word.slice(1)
            })
            .join(" ")
    }

    const [Menu, currentPage] = useSideMenu()
    const Header = <div>{toNormalRepresentation(currentPage)}</div>

    return (
        <Themed>
            <div className="min-w-[800px] overflow-auto">
                <Layout menu={Menu} header={Header}>
                    <LayoutItem span={3}>
                        <LayoutItemBody>
                            {currentPage == "relying-party" && (
                                <RelyingParty token={token} />
                            )}
                        </LayoutItemBody>
                    </LayoutItem>
                </Layout>
            </div>
        </Themed>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
