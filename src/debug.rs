#![allow(unused_macros, dead_code)]

use std::collections::HashSet;
use std::cell::RefCell;

thread_local! {
    // Not perfect as it might print multiple times (once per thread),
    // but better than a `global` hack to prevent multiple prints of the
    // same warning.
    pub(crate) static OUTPUT_GUARD: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}


macro_rules! debug_append_log {
    ($str:expr) => {
        use std::io::Write;

        // Allows the user to configure debug file path at compile time,
        // e.g., when building for embedded / Android.
        let file_name = option_env!("FASTER_DEBUG_FILE").unwrap_or("faster-debug.txt");

        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_name).and_then(|mut file| {
                writeln!(file, "{}", $str)
        }).ok(); // `ok` suppresses warning about unused results, about which we don't care.
    }
}


/// Prints the given string once (for the current thread).
/// Useful for not spamming the console.
macro_rules! debug_output_once {
    ($str:expr) => {
        let output = $str;

        crate::debug::OUTPUT_GUARD.with(|f| {
            let mut output_guard = f.borrow_mut();

            if output_guard.contains(&output) {
                return;
            }

            // Also print to file (if enabled).
            debug_append_log!(output);
            println!("{}", output);

            output_guard.insert(output);
        });
    }
}


/// Signal a software fallback is executed.
#[cfg(feature="trace")]
macro_rules! fallback {
    () => {
        debug_output_once!(format!("⛔ faster is using SOFTWARE emulation here ({}:{}).", file!(), line!()));
    }
}

/// Signal an optimized SIMD intrinsic is executed.
#[cfg(feature="trace")]
macro_rules! optimized {
    () => {
        debug_output_once!(format!("🚄 faster is using HARDWARE acceleration here ({}:{}).", file!(), line!()));
    }
}

#[cfg(not(feature="trace"))]
macro_rules! fallback {
    () => { }
}

#[cfg(not(feature="trace"))]
macro_rules! optimized {
    () => { }
}

