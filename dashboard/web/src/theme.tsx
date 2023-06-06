import React from "react"
import styled, { ThemeProvider } from "styled-components"

import { CharcoalTheme, light } from "@charcoal-ui/theme"
import { TokenInjector, createTheme } from "@charcoal-ui/styled"

declare module "styled-components" {
    export interface DefaultTheme extends CharcoalTheme {}
}

const Themed = (props: { children: React.ReactNode }) => (
    <ThemeProvider theme={light}>
        <TokenInjector theme={{ ":root": light }} />
        {props.children}
    </ThemeProvider>
)

const theme = createTheme(styled)

export { Themed, theme }
