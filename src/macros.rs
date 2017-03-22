macro_rules! wlnerr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
);

macro_rules! werr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        write!(&mut stderr(), $($arg)*).ok();
    })
);

macro_rules! verbose(
    ($cfg:expr, $($arg:tt)*) => ({
        if $cfg.verbose {
            use std::io::{Write, stdout};
            write!(&mut stdout(), $($arg)*).ok();
        }
    })
);

macro_rules! verboseln(
    ($cfg:expr, $($arg:tt)*) => ({
        if $cfg.verbose {
            use std::io::{Write, stdout};
            writeln!(&mut stdout(), $($arg)*).ok();
        }
    })
);

#[cfg(feature = "debug")]
macro_rules! debugln {
    ($fmt:expr) => (println!(concat!("DEBUG:cargo-count: ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("DEBUG:cargo-count: ",$fmt), $($arg)*));
}

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt:expr) => (print!(concat!("DEBUG:cargo-count: ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("DEBUG:cargo-count: ",$fmt), $($arg)*));
}

#[cfg(not(feature = "debug"))]
macro_rules! debugln {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}
