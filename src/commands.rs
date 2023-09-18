use libsums::member::StudentId;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
pub async fn verify(
    ctx: Context<'_>,
    #[description = "Your Student ID"] student_id: StudentId,
) -> Result<(), Error> {
    ctx.say(format!("Your student ID is {student_id}")).await?;
    Ok(())
}
