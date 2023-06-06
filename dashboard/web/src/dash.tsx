import { Layout, LayoutItem, LayoutItemBody } from "@charcoal-ui/react-sandbox"
import React, { StrictMode, Suspense, lazy, useEffect, useState } from "react"
import { createRoot } from "react-dom/client"
import { Themed } from "./theme"
import { useSideMenu } from "./useSideMenu"

import "../main.css"

const Loading = () => <div>Loading</div>

const RelyingParty = lazy(() => import("./relyingParty"))
const User = lazy(() => import("./user"))
const Role = lazy(() => import("./role"))

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
                                    {currentPage == "role" && <Role />}
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
