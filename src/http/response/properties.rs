pub trait ResponseProperties<'rp> {
    const SERVER_NAME: &'rp str;
    const PROTOCOL_VERSION: &'rp str;
}