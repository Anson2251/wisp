use rusqlite::Statement;

/// Leak a `Statement` to make it `'static`.
///
/// **MUST MAKE SURE** the statement would be dropped before the connection is closed.
pub unsafe fn leak_stmt<'a>(stmt: Statement<'a>) -> Statement<'static> {
    std::mem::transmute(stmt) // "Forget" the lifetime, pretend it's 'static
}
