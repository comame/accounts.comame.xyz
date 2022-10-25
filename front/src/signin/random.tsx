export function random(length: number) {
    const bytes = new Uint8Array(Math.trunc(length * 3 / 4) + 1).fill(0)
    crypto.getRandomValues(bytes)
    const rand = btoa(toHexString(bytes))
    return rand.slice(0, length)
}

function toHexString(byteArray: Uint8Array) {
    return Array.from(byteArray, function(byte) {
      return ('0' + (byte & 0xFF).toString(16)).slice(-2);
    }).join('')
  }
