use crate::{cli::ContestArgs, App};

pub async fn contest(app: &App, args: &ContestArgs) -> crate::Result<()> {
    println!("Contest called");
    println!("{:?}", &args.yes);
    Ok(())
}
