/// A shorthand for `Default::default()` that you can `use default::default`.
///
/// See https://internals.rust-lang.org/t/could-we-have-std-default/8756 for
/// discussion
pub fn default<T:Default>()-> T {
   std::default::Default::default() // gosh, that's a lot of default, isn't it?
}
