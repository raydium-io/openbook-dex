mod middleware;
mod proxy;

use anchor_lang::declare_id;

// Serum DEX program ID (mainnet)
declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");

pub use middleware::*;
pub use proxy::*;
pub use serum_dex;
