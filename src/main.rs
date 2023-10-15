mod commands;

use std::{env, sync::Mutex};

use commands::{verify, Data};
use poise::serenity_prelude::{GatewayIntents, RoleId};

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![verify()],
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN"))
        .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    role_id: RoleId(
                        env::var("ROLE_ID")
                            .expect("Missing ROLE_ID")
                            .parse()
                            .expect("Invalid ROLE_ID"),
                    ),
                    members: Mutex::new(vec![]),
                    group_id: env::var("GROUP_ID")
                        .expect("Missing GROUP_ID")
                        .parse()
                        .expect("Invalid GROUP_ID"),
                    webdriver_address: env::var("WEBDRIVER_ADDRESS")
                        .expect("Missing WEBDRIVER_ADDRESS"),
                    sums_username: env::var("SUMS_USERNAME").expect("Missing SUMS_USERNAME"),
                    sums_password: env::var("SUMS_PASSWORD").expect("Missing SUMS_PASSWORD"),
                    browser_name: env::var("BROWSER_NAME").expect("Missing BROWSER_NAME"),
                })
            })
        });

    framework.run().await.unwrap();
}
