#[lang = "sized"]
pub trait Sized {}

mod option {
	enum Option<T> {
	    #[lang = "None"]
	    None,
	    #[lang = "Some"]
	    Some(T),
	}
}

use option::Option::{self, None, Some};

fn main() {
    let Some(x) = Some(1) else {
		return;
    };
	if x == 1 {
		x += 1;
	}
}
