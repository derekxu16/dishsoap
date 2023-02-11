mod environment;
mod type_checker;

pub use environment::*;
pub use type_checker::*;

#[cfg(test)]
mod tests;
