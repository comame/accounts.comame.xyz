import styled, { createGlobalStyle } from "styled-components"
import { theme } from "../theme"

export const Global = createGlobalStyle`
    html {
        ${theme(o => [
            o.bg.surface3,
        ])}
        font-family: sans-serif;
    }
`

export const TextContainer = styled.div`
    line-height: 2;

    ${theme(o => [
        o.margin.bottom(40),
        o.font.text1,
    ])}
`

export const Bold = styled.span`
    font-weight: bold;
`

export const InputContainer = styled.div`
    display: grid;
    gap: ${ ({ theme }) => theme.spacing[24] }px;

    ${theme(o => [
        o.margin.bottom(40),
    ])}
`

export const ButtonsContainer = styled.div`
    display: grid;
    gap: ${ ({ theme }) => theme.spacing[24] }px;

    ${theme(o => [
        o.margin.bottom(40),
    ])}
`
