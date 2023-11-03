mod access;
mod accounts;
mod changes;
mod client;
mod config;
pub mod documentation;
mod groups;
mod plugins;
mod projects;

pub use client::GerritClient;

pub fn add(left: usize, right: usize) -> usize {
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
