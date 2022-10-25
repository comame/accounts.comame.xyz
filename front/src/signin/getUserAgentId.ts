import { random } from "./random"

export function getUserAgentId(): string {
    const current = localStorage.getItem('ua-id')
    if (current) {
        return current
    }

    const randomStr = random(64)
    localStorage.setItem('ua-id', randomStr)
    return randomStr
}
