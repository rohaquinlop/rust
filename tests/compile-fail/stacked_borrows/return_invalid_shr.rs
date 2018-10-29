// Make sure that we cannot return a `&` that got already invalidated.
fn foo(x: &mut (i32, i32)) -> &i32 {
    let xraw = x as *mut (i32, i32);
    let ret = unsafe { &(*xraw).1 };
    x.1 = 42; // invalidate xraw on the 2nd field
    ret //~ ERROR Shr reference with non-reactivatable tag: Location should be frozen
}

fn main() {
    foo(&mut (1, 2));
}
