pub mod config;
pub mod daemon;
pub mod db;
pub mod grpc;
pub mod helpers;
pub mod zwave;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
