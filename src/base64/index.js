function encode(str) {
    return Buffer.from(str).toString('base64');
}

function decode(str) {
    return Buffer.from(str, 'base64').toString();
}

module.exports = {
    encode,
    decode
};
