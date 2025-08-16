pub fn now_nanos() -> u64 {
    ic_cdk::api::time()
}

pub fn now_seconds() -> u64 {
    now_nanos() / 1_000_000_000
}