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

macro_rules! atoms_index {
    ( $first:ident $( $atom:ident )* ) => {
        pub const $first: usize = 0;

        atoms_index!($($atom)*, 0);
    };
    ( $first:ident $( $atom:ident )*, $index:expr ) => {
        pub const $first: usize = $index + 1;

        atoms_index!($($atom)*, $index + 1);
    };
    ( , $index:expr ) => ();
}

macro_rules! atoms {
    ( $( $atom:ident ),* $( , )? ) => {
        atoms_index!($($atom)*);

        pub const ATOMS: [&str; count!($($atom)*)] = [$(stringify!($atom),)*];
    };
}
