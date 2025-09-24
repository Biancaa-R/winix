pub mod ansi;
pub mod cat;
#[cfg(windows)]
pub mod chmod;
pub mod chown;
pub mod df;
pub mod disown;
pub mod env;
pub mod echo;
pub mod free;
pub mod git;
pub mod grep;
pub mod head;
pub mod input;
pub mod kill;
pub mod nproc;
pub mod pipeline;
pub mod powershell;
pub mod process;
pub mod ps;
pub mod rm;
pub mod sensors;
pub mod sudo;
pub mod tail;
pub mod touch;
pub mod tui;
pub mod uname;
pub mod uptime;

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(1 + 1, 2);
    }
}
