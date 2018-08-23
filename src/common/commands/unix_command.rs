//! This module contains the commands that can be used for unix systems.

use super::{ IStateCommand};
use kernel::unix_kernel::terminal;
use termios::{tcsetattr, Termios, CREAD, ECHO, ICANON, TCSAFLUSH};

use std::sync::{Once, ONCE_INIT};
static TERMINAL_MODE: Once = ONCE_INIT;

const FD_STDIN: ::std::os::unix::io::RawFd = 1;

use std::io::{Error, ErrorKind, Result};

/// This command is used for switching to NoncanonicalMode.
#[derive(Copy, Clone)]
pub struct NoncanonicalModeCommand;

impl NoncanonicalModeCommand {
    pub fn new() -> NoncanonicalModeCommand {
        NoncanonicalModeCommand {}
    }
}

impl NoncanonicalModeCommand {
    pub fn enable(&mut self) -> Result<()> {
        // Set noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN) {
            let mut noncan = orig.clone();
            noncan.c_lflag &= !ICANON;
            noncan.c_lflag &= !ECHO;
            noncan.c_lflag &= !CREAD;
            tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }
        Ok(())
    }

    pub fn disable(&self) -> Result<()> {
        // Disable noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN) {
            let mut noncan = orig.clone();
            noncan.c_lflag &= ICANON;
            noncan.c_lflag &= ECHO;
            noncan.c_lflag &= CREAD;

            tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }
        Ok(())
    }
}

// This command is used for enabling and disabling raw mode for the terminal.
/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand;

impl RawModeCommand {
    pub fn new() -> Self {
        return RawModeCommand {}
    }

    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        terminal::into_raw_mode();
        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&mut self) -> Result<()> {
       terminal::disable_raw_mode();
        Ok(())
    }
}

