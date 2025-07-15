use crate::{
    error::Result,
    cli::{Shell, Cli},
};
use clap::CommandFactory;
use clap_complete::{generate, shells};
use std::io;

pub struct CompletionsCommand;

impl CompletionsCommand {
    pub fn execute(shell: Shell) -> Result<()> {
        let mut cmd = Cli::command();
        
        match shell {
            Shell::Bash => {
                generate(shells::Bash, &mut cmd, "edisonprompt", &mut io::stdout());
            }
            Shell::Zsh => {
                generate(shells::Zsh, &mut cmd, "edisonprompt", &mut io::stdout());
            }
            Shell::Fish => {
                generate(shells::Fish, &mut cmd, "edisonprompt", &mut io::stdout());
            }
            Shell::PowerShell => {
                generate(shells::PowerShell, &mut cmd, "edisonprompt", &mut io::stdout());
            }
        }
        
        Ok(())
    }
}