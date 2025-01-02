import styled, { createGlobalStyle } from "styled-components"

export const Global = createGlobalStyle`
    html {
        @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
            background-color: ${(props) => props.theme.color.surface3};
        }
        font-family: sans-serif;
    }
`

export const TextContainer = styled.div`
    line-height: 2;

    margin-bottom: ${(props) => props.theme.spacing[40]}px;
    color: ${(props) => props.theme.color.text1};
`

export const Bold = styled.span`
    font-weight: bold;
`

export const InputContainer = styled.div`
    display: grid;
    gap: ${({ theme }) => theme.spacing[24]}px;

    margin-bottom: ${(props) => props.theme.spacing[40]}px;
`

export const ButtonsContainer = styled.div`
    display: grid;
    gap: ${({ theme }) => theme.spacing[24]}px;

    margin-bottom: ${(props) => props.theme.spacing[40]}px;
`

export const Layout = styled.div`
    @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
        display: grid;
        justify-items: center;
        padding: ${(props) => props.theme.spacing[40]}px 0;
    }
`

export const LayoutItem = styled.div`
    background-color: ${(props) => props.theme.color.background1};
    padding: 0 ${(props) => props.theme.spacing[16]}px;

    @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
        width: 600px;
        padding: 0 ${(props) => props.theme.spacing[40]};
        background-color: ${(props) => props.theme.color.background1};
        border-radius: ${(props) => props.theme.borderRadius[24]}px;
    }
`

export const LayoutItemHeader = styled.div`
    background-color: ${(props) => props.theme.color.background1};
    height: ${(props) => props.theme.spacing[40]}px;
    line-height: ${(props) => props.theme.spacing[40]}px;

    @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
        background-color: ${(props) => props.theme.color.surface2};
        padding: 0 ${(props) => props.theme.spacing[40]}px;
        height: ${(props) => props.theme.spacing[64]}px;
        line-height: ${(props) => props.theme.spacing[64]}px;
        border-top-left-radius: ${(props) => props.theme.borderRadius[24]}px;
        border-top-right-radius: ${(props) => props.theme.borderRadius[24]}px;
    }

    font-weight: bold;
    text-align: center;
    color: ${(props) => props.theme.color.text2};
`

export const LayoutItemBody = styled.div`
    background-color: ${(props) => props.theme.color.background1};
    padding-top: ${(props) => props.theme.spacing[16]}px;

    @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
        background-color: ${(props) => props.theme.color.background1};
        padding: ${(props) => props.theme.spacing[40]}px;
        border-bottom-left-radius: ${(props) => props.theme.borderRadius[24]}px;
        border-bottom-right-radius: ${(props) =>
            props.theme.borderRadius[24]}px;
    }
`
