
use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::interaction::application_command::CommandDataOption,
};

pub fn run(_options: &[CommandDataOption]) -> String {
	"According to all known laws of aviation, there is no way a bee should be able to fly.\nIts wings are too small to get its fat little body off the ground.".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("bees").description("Do You Like Jazz?")
}
