use core::error::Error;

/// A general-purpose function for results that we don't know what to do with.
///
/// For instance, if they occur during Drop or other inconvenient places such as system callbacks.
/// At the very least this allows us to have better control over the overall behavior when errors
/// occur in such a context.
pub fn consume<T, E: Error>(r: Result<T, E>) {
    drop(inspect(r));
}

/// A general-purpose function for results that we don't know what to do with.
///
/// For instance, if they occur during Drop or other inconvenient places such as system callbacks.
/// At the very least this allows us to have better control over the overall behavior when errors
/// occur in such a context.
pub fn inspect<T, E: Error>(r: Result<T, E>) -> Result<T, E> {
    if let Err(e) = r.as_ref() {
        eprintln!("Discarded result: {e}");
        debugger::breakpoint!();
    }
    r
}

