use serde::{Deserialize, Serialize};

#[dervive(Serialize, Deserialize, Debug)]
struct BaseUri {
    service: String,
    account: String,
    baseURI: String,
}
