mod non_zeros;
mod zeros;

pub use self::{
    non_zeros::factorial_tails,
    zeros::{factorial_zeros, factorial_zeros_fast},
};
