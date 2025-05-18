#[define_in_every_cgu_used] //~ ERROR: the `#[define_in_every_cgu_used]` attribute is an experimental feature
static STATIC: i32 = 0;

fn main() {}
