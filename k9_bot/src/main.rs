mod structs;

use poise::serenity_prelude as serenity; //this is used to access permissions and users / channels

use poise::Framework; // main builder

use poise::FrameworkOptions; // command prefix and registering commands

use sqlx::SqlitePool; //used to access database

use tokio;

use structs::{Data,Quote,Episode};

use dotenv::dotenv;

use std::fs;

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


//imported all the crates and structures used in da bot


#[tokio::main] //this is here to tel the main function to be able to be ran by multiple users asycronioushljtrlt
async fn main() -> Result<(), Error> {


    println!("Hello, world!");



    Ok(())
}
