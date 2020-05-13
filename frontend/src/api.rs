macro_rules! load_queries {
    ($($name:tt,)*) => {$(
        include!(concat!(env!("OUT_DIR"), "/queries/", $name, ".rs"));
    )*};
}

load_queries! {}
