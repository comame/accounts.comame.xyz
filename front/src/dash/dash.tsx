import { Layout, LayoutItem, LayoutItemBody } from "@charcoal-ui/react-sandbox"
import React, { StrictMode, Suspense, useEffect, useState } from "react"
import { createRoot } from "react-dom/client"
import { Themed } from "../theme"
import { RelyingParty } from "./relyingParty"
import { User } from "./user"
import { useSideMenu } from "./useSideMenu"

const Loading = () => <div>Loading</div>

const App = () => {
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
        <StrictMode>
            <Themed>
                <div className="min-w-[800px] overflow-auto">
                    <Layout menu={Menu} header={Header}>
                        <LayoutItem span={3}>
                            <LayoutItemBody>
                                <Suspense fallback={<Loading />}>
                                    {currentPage == "relying-party" && (
                                        <RelyingParty />
                                    )}
                                    {currentPage == "user" && <User />}
                                </Suspense>
                            </LayoutItemBody>
                        </LayoutItem>
                    </Layout>
                </div>
            </Themed>
        </StrictMode>
    )
}

createRoot(document.getElementById("app")!).render(<App />)
