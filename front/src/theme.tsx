import React from "react"
import { CharcoalProvider } from "@charcoal-ui/react"

import "@charcoal-ui/react/dist/layered.css"
import { TokenInjector } from "@charcoal-ui/styled"
import { CharcoalTheme, light } from "@charcoal-ui/theme"
import { ThemeProvider } from "styled-components"

declare module "styled-components" {
    export interface DefaultTheme extends CharcoalTheme {}
}

const Themed = (props: { children: React.ReactNode }) => (
    <CharcoalProvider>
        <TokenInjector theme={{ ":root": light }}></TokenInjector>
        <ThemeProvider theme={light}>{props.children}</ThemeProvider>
    </CharcoalProvider>
)

export { Themed }
