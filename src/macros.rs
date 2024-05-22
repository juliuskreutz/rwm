macro_rules! spawn {
    ( $command:expr ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.spawn($command, &[])
    };
    ( $command:expr, $( $arg:expr ),* $( , )? ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.spawn($command, &[$($arg,)*])
    };
}

macro_rules! kill {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.kill()
    };
}

macro_rules! swap {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.swap()
    };
}

macro_rules! main_factor {
    ( $factor:expr ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.main_factor($factor)
    };
}

macro_rules! toggle_fullscreen {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.toggle_fullscreen()
    };
}

macro_rules! toggle_floating {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.toggle_floating()
    };
}

macro_rules! view {
    ( $tag:expr ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.view($tag)
    };
}

macro_rules! tag {
    ( $tag:expr ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.tag($tag)
    };
}

macro_rules! tagmon {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.tagmon()
    };
}

macro_rules! move_up {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.move_up()
    };
}

macro_rules! move_down {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.move_down()
    };
}

macro_rules! quit {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.quit()
    };
}

macro_rules! drag {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.drag()
    };
}

macro_rules! resize {
    () => {
        |rwm: &mut $crate::rwm::Rwm| rwm.resize()
    };
}

macro_rules! count {
    () => (0);
    ( $x:tt $($xs:tt)* ) => (1 + count!($($xs)*));
}

macro_rules! tags {
    ( $( $tag:expr ),* $( , )? ) => {
        pub const TAGS: [&str; count!($($tag)*)] = [$($tag,)*];
    };
}

macro_rules! keys {
    ( $( $tup:expr ),* $( , )? ) => {
        pub const KEYS: [($crate::combo::KeyCombo, fn(&mut $crate::rwm::Rwm)); count!($($tup)*)] = [$(($crate::combo::KeyCombo::new($tup.0, $tup.1), $tup.2)),*];
    };
}

macro_rules! buttons {
    ( $( $tup:expr ),* $( , )? ) => {
        pub const BUTTONS: [($crate::combo::ButtonCombo, fn(&mut $crate::rwm::Rwm)); count!($($tup)*)] = [$(($crate::combo::ButtonCombo::new($tup.0, $tup.1), $tup.2)),*];
    };
}

macro_rules! atoms {
    (
        $(
            $field:ident => $name:tt,
        )*
    ) => {
        struct Atoms {
            $(
                $field: xcb::x::Atom,
            )*
            all: Vec<xcb::x::Atom>,
        }

        impl Atoms {
            fn new(conn: &xcb::Connection) -> xcb::Result<Atoms> {
                $(
                    let $field = conn.send_request(&xcb::x::InternAtom {
                        only_if_exists: false,
                        name: $name,
                    });
                )*
                $(
                    let $field = conn.wait_for_reply($field)?.atom();
                )*
                let all = vec![
                    $(
                        $field,
                    )*
                ];
                Ok(Atoms {
                    $(
                        $field,
                    )*
                    all
                })
            }
        }
    };
}
pub(crate) use atoms;
