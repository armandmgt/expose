export class RandomToken {
  /**
   * Returns the hexadecimal representation of an integer
   * @param {Number} dec - The integer to convert
   * @return {string}
   */
  static dec2hex(dec) {
    return dec.toString(16).padStart(2, "0")
  }

  /**
   * Generates a random hex string of length {@param len}
   * @param {Number, undefined} len - The desired length
   * @return {string}
   */
  static generateId(len= 40) {
    const arr = new Uint8Array(len / 2)
    window.crypto.getRandomValues(arr)
    return Array.from(arr, this.dec2hex).join('')
  }
}
