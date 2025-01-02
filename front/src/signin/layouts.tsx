import styled, { createGlobalStyle } from "styled-components"

export const Global = createGlobalStyle`
    html {
        @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
            background-color: ${(props) => props.theme.color.surface3};
        }
        font-family: sans-serif;
    }
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

    @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
        width: 600px;
        background-color: ${(props) => props.theme.color.background1};
        border-radius: ${(props) => props.theme.borderRadius[24]}px;
    }
`

export const LayoutItemHeader = styled.div`
    background-color: ${(props) => props.theme.color.background1};
    height: ${(props) => props.theme.spacing[40]}px;
    line-height: ${(props) => props.theme.spacing[40]}px;
    padding: 0 ${(props) => props.theme.spacing[16]}px;

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
    padding: 0 ${(props) => props.theme.spacing[16]}px;
    padding-top: ${(props) => props.theme.spacing[16]}px;

    @media (min-width: ${(props) => props.theme.breakpoint.screen1}px) {
        background-color: ${(props) => props.theme.color.background1};
        padding: ${(props) => props.theme.spacing[40]}px;
        border-bottom-left-radius: ${(props) => props.theme.borderRadius[24]}px;
        border-bottom-right-radius: ${(props) =>
            props.theme.borderRadius[24]}px;
    }
`
