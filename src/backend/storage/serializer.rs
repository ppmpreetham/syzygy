use super::Exchange;

// TODO

pub(super) fn serialize(ex: &Exchange) -> Vec<u8> {
    // bincode/postcard over a plain StoredExchange struct,  only runs at flush time
    // bodies stored as raw Vec<u8> via ex.request.body().clone().into_inner()
    todo!()
}

pub(super) fn deserialize(bytes: &[u8]) -> Option<Exchange> {
    // bincode/postcard over a plain StoredExchange struct,  only runs at flush time
    // bodies stored as raw Vec<u8> via ex.request.body().clone().into_inner()
    todo!()
}
