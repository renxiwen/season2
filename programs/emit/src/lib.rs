use anchor_lang::prelude::*;

declare_id!("FANaQzvSCrh6kzT1Ak9vvuwm4LCwd6ZDEXdTcFbc2YaL");

#[program]
pub mod emit_event_mod {
    use super::*;

    pub fn emit_event(_ctx: Context<EmitEvent>,input: String) -> Result<()> {
        emit!(CustomEvent{message: input});
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EmitEvent {}


#[event]
pub struct CustomEvent {
    pub message: String,
}