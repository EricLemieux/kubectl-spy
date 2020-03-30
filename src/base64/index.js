/**
 * Encode string into base64 version of that string.
 *
 * @param {string} str
 * @returns {string}
 */
function encode(str) {
    return Buffer.from(str).toString('base64');
}

/**
 * Decode base64 string into decoded string.
 *
 * @param {string} str
 * @returns {string}
 */
function decode(str) {
    return Buffer.from(str, 'base64').toString();
}

module.exports = {
    encode,
    decode
};
