use std::env;

use serenity::{
	async_trait,
	model::{
		application::{
			command::Command,
			interaction::{
				Interaction,
				InteractionResponseType
			},
		},
		// id::GuildId,
		gateway::Ready,
		prelude::*,
	},
	prelude::{EventHandler, Context},
	Client,
};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(
		&self,
		ctx: Context,
		interaction: Interaction
	) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Recieved command interaction: {:#?}", command.data.name);

			let content = match command.data.name.as_str() {
				"bees" => commands::bees::run(&command.data.options),
				"ping" => commands::ping::run(&command.data.options),
				_ => "not implemented!".to_string(),
			};

			if let Err(why) = command
				.create_interaction_response(&ctx.http, |response| {
					response
						.kind(InteractionResponseType::ChannelMessageWithSource)
						.interaction_response_data(|message| message.content(content))
				})
				.await
			{
				println!("Cannot respon to slash command: {}", why);
			}
		}
	}

	async fn ready(
		&self,
		ctx: Context,
		ready: Ready,
	) {
		println!("{} is connected!", ready.user.name);

		// let guild_id = GuildId(
		// 	env::var("GUILD_ID")
		// 		.expect("Exptected GUILD_ID envitoment variable")
		// 		.parse()
		// 		.expect("GUILD_ID must be an integer"),
		// );

		let _commands = Command::set_global_application_commands(&ctx.http, |commands| {
			commands
				.create_application_command(|command| commands::ping::register(command))
				.create_application_command(|command| commands::wonderful_command::register(command))
				.create_application_command(|command| commands::bees::register(command))
		})
			.await;
	}
}

#[tokio::main]
async fn main() {
	// Configure the client with your Discord bot token in the enviroment.
	let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN enviroment variable");

	// Build our client
	let mut client = Client::builder(token, GatewayIntents::empty())
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	// Finally, start a single shard, and start listening to events.
	//
	// Shards will automatically attempt to reconnect,
	// and will perform backoff until it reconnects.
	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}
