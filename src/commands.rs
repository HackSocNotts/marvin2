use std::sync::Mutex;

use libsums::{
    client::SumsClient,
    member::{Member, StudentId},
};
use poise::serenity_prelude::RoleId;

pub struct Data {
    pub role_id: RoleId,
    pub members: Mutex<Vec<Member>>,
    pub group_id: u16,
    pub webdriver_address: String,
    pub sums_username: String,
    pub sums_password: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral)]
pub async fn verify(
    ctx: Context<'_>,
    #[description = "Your Student ID"] student_id: StudentId,
) -> Result<(), Error> {
    // We do these awkward scoped accesses to members because Rust doesn't like
    // holding mutexes past async calls
    let is_member = {
        let members = ctx.data().members.lock().unwrap();
        members
            .iter()
            .any(|student| student.student_id == student_id)
    };

    if is_member {
        add_role(ctx).await?;

        return Ok(());
    }

    ctx.defer_ephemeral().await?;

    let client = SumsClient::new(ctx.data().group_id, &ctx.data().webdriver_address).await?;

    client
        .authenticate(&ctx.data().sums_username, &ctx.data().sums_password)
        .await?;

    let new_student_list = client.members().await?;

    let is_now_in_members = new_student_list
        .iter()
        .any(|student| student.student_id == student_id);

    *ctx.data().members.lock().unwrap() = new_student_list;

    if is_now_in_members {
        add_role(ctx).await?;

        return Ok(());
    }

    ctx.say("Not found :(").await?;

    Ok(())
}

async fn add_role(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Verified! Welcome to HackSoc :)").await?;

    if let Some(mut author) = ctx.author_member().await {
        author
            .to_mut()
            .add_role(ctx.http(), ctx.data().role_id)
            .await?;
    }

    Ok(())
}
