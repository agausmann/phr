macro_rules! load_queries {
    ($($name:tt,)*) => {$(
        include!(concat!(env!("OUT_DIR"), "/", $name, ".rs"));
    )*};
}

load_queries! {}
