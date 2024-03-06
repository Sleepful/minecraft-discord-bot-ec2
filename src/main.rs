use poise::serenity_prelude as serenity;

mod aws;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn turn_on(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let _u = user.as_ref().unwrap_or_else(|| ctx.author());
    // stdout is from commmand is sent to stdout of program
    let previous_state = aws::aws_start_cmd();

    match previous_state.as_str() {
        "running\n" => {
            let ip = aws::aws_ip_cmd();
            let response = format!("The server is already running at IP:\n- {}", ip);
            ctx.say(response).await?;
            Ok(())
        }
        _ => {
            let response = format!("Starting the server.");
            ctx.say(response).await?;
            Ok(())
        }
    }
}

#[poise::command(slash_command, prefix_command)]
async fn server_state(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let _u = user.as_ref().unwrap_or_else(|| ctx.author());
    // aws ec2 describe-instances --instance-ids ${MC_INSTANCE_ID}
    // | jq -r '.Reservations[].Instances[].State.Name'
    let result = aws::aws_state_cmd();
    match result.as_str() {
        "running\n" => {
            let ip = aws::aws_ip_cmd();
            let response = format!("The server is already running at IP:\n- {}", ip);
            ctx.say(response).await?;
            Ok(())
        }
        _ => {
            let response = format!("The server is: {}", result);
            ctx.say(response).await?;
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let _ = std::env::var("INSTANCE_ID").expect("missing INSTANCE_ID");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![turn_on(), server_state()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
