//! Syphon is a Rust wrapper around the [Syphon Framework](https://syphon.github.io/).
//! It will only support whatever configuration and utilities needed by the
//! MetalSyphonServer.

pub mod metal_server;
mod server_base;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
