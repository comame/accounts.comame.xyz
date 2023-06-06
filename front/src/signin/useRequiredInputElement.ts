import { useEffect } from "react"

/**
 * TextField が required を input に渡さない問題に対するワークアラウンド
 */
export function useRequiredInputElement(deps: any[]) {
    useEffect(() => {
        document.querySelectorAll("input[aria-required=true]").forEach((v) => {
            v.setAttribute("required", "true")
        })
    }, [...deps])
}
