// @ts-check

/**
 * @param {string} base64
 * @returns {Uint8Array}
 */
export function decodeBase64URIToUint8Array(base64) {
  const words = [];
  for (const c of base64) {
    switch (true) {
      case "A" <= c && c <= "Z":
        words.push(c.charCodeAt(0) - 65);
        break;
      case "a" <= c && c <= "z":
        words.push(c.charCodeAt(0) - 71);
        break;
      case "0" <= c && c <= "9":
        words.push(c.charCodeAt(0) + 4);
        break;
      case c == "-":
        words.push(0b111110);
        break;
      case c == "_":
        words.push(0b111111);
        break;
    }
  }

  // 123456 12 3456 1234 56 123456
  // 123456 78 1234 5678 12 345678
  const bytes = [];
  for (let i = 0; i < words.length; i += 1) {
    const lastBytesIndex = bytes.length - 1;
    switch (i % 4) {
      case 0:
        bytes.push(words[i]);
        break;
      case 1:
        bytes[lastBytesIndex] =
          (bytes[lastBytesIndex] << 2) + ((words[i] & 0b110000) >> 4);
        bytes.push(words[i] & 0b001111);
        break;
      case 2:
        bytes[lastBytesIndex] =
          (bytes[lastBytesIndex] << 4) + ((words[i] & 0b111100) >> 2);
        bytes.push(words[i] & 0b000011);
        break;
      case 3:
        bytes[lastBytesIndex] = (bytes[lastBytesIndex] << 6) + words[i];
    }
  }

  const messageLength = Math.trunc((words.length * 6) / 8);
  return new Uint8Array(bytes.slice(0, messageLength));
}
