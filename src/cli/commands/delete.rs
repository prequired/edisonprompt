use crate::{
    database::Database,
    error::Result,
    utils,
};

pub struct DeleteCommand;

impl DeleteCommand {
    pub fn execute(
        database: &mut Database,
        name: String,
        yes: bool,
        force: bool,
    ) -> Result<()> {
        // Check if prompt exists first
        let _prompt = database.get_prompt(&name)?;
        
        // Confirm deletion unless --yes or --force is used
        if !yes && !force {
            if !utils::confirm(&format!("Delete prompt '{}'? This cannot be undone", name))? {
                utils::print_info("Delete cancelled");
                return Ok(());
            }
        }
        
        // Delete the prompt
        database.delete_prompt(&name)?;
        
        utils::print_success(&format!("Deleted prompt '{}'", name));
        
        Ok(())
    }
}