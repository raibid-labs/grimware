// Library entry point for mobile platforms
#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub use mobile::*;

pub fn run() {
    // Mobile-specific initialization can go here
}
