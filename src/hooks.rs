use crate::types::types;
macro_rules! with_params {
    (&mut $entry: tt; param $name: literal; $value: expr; $param_type:ty ) => {
        crate::trace::TraceEntry::with_param::<$param_type>(&mut $entry, $name, $value);


    };
    (&mut $entry: tt; param $name: literal; $value: expr; $param_type:ty, $(param $next_name: literal; $next_value: expr; $next_param_type:ty),+) => {
        with_params!(&mut $entry; param $name; $value; $param_type);
        with_params!(&mut $entry; $(param $next_name; $next_value; $next_param_type),+ )
    };
    (&mut $entry:tt;) => {
        ()
    };
}

include!(concat!(env!("OUT_DIR"), "/hooks.rs"));
