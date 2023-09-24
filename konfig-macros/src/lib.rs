mod konfig;

use proc_macro::TokenStream;

#[proc_macro]
#[inline]
pub fn konfig(arg: TokenStream) -> TokenStream {
    self::konfig::expand(arg)
}
