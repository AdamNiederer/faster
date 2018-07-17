use std::collections::HashSet;
use std::cell::RefCell;

thread_local! {
    // Not perfect, but better than a `global` hack to prevent multiple prints
    // of the same warning.
    pub(crate) static OUTPUT_GUARD: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}


/// Prints the given string once (for the current thread).
/// Useful for not spamming the console.
macro_rules! debug_print_once {
    ($str:expr) => {
        let output = $str;

        crate::debug::OUTPUT_GUARD.with(|f| {
            let mut output_guard = f.borrow_mut();

            if output_guard.contains(&output) {
                return;
            }

            println!("{}", output);

            output_guard.insert(output);
        });
    }
}


macro_rules! fallback {
    () => {
        debug_print_once!(format!("⛔ faster is using SOFTWARE emulation here ({}:{}).", file!(), line!()));
    }
}


macro_rules! optimized {
    () => {
        debug_print_once!(format!("🚄 faster is using HARDWARE acceleration here ({}:{}).", file!(), line!()));
    }
}
