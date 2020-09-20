use shared::structs::Config;

struct CSDSClient {
    getAll: fn(Future<T>),

    requestHandler: fn(),

    convert: fn(HashMap),
}

impl CSDSClient {
    fn new(conf: Config) {}
}
