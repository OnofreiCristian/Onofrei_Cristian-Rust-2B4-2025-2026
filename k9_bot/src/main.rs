mod structs;

use poise::serenity_prelude as serenity; //this is used to access permissions and users / channels
use poise::Framework; // main builder
use poise::FrameworkOptions; use rand::random;
use sqlx::Sqlite;
// command prefix and registering commands
use sqlx::SqlitePool; //used to access database
use std::fs; //to read JSON files
use std::env; //to read enviroment files
use std::sync::Mutex; // to share trivia state
use dotenv::dotenv; // to load the .env file

use structs::{Data,Quote,Episode};

//using this to choose random quotes
use rand::rng;
use rand::prelude::IndexedRandom;


// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


//imported all the crates and structures used in da bot


//a test command to see if it works

#[poise::command(prefix_command)]
async fn ping(ctx: Context <'_>) -> Result<(),Error>{


    ctx.say("Pong!").await?;

    Ok(())

}


#[poise::command(prefix_command)]
pub async fn quote(ctx: Context<'_>) -> Result<(),Error>
{

    let data = ctx.data(); //my data is the vector of quotes, so we are accessing them directly

    
    //opening new sope so rng is created and accessed
    //if not, .await would freak out
    let random_quote = {
        let mut r = rng(); //choosing a temporary random number
        data.quotes.choose(&mut r).cloned() //choosing a random quite using the random number,
    };



    if let Some(quote) = random_quote 
    {
        let response = format!(

            "\"{}\" \n- **{}**{}",
            quote.text,
            quote.Doctor,
            quote.Source.as_ref().map(|s| format!(" ({})", s)).unwrap_or_default() 
            //as_ref = as reference, map creates paranthases if source exists, unwrap returns an empty string if theres no source
        );


        //formated the response

        ctx.say(response).await?;
        

    }
    else {
        ctx.say("Sorry, Couldn't find any quotes.").await?;
    }


    Ok(())

}


#[tokio::main] //this is here to tel the main function to be able to be ran by multiple users asycronioushljtrlt
async fn main() -> Result<(), Error> {

    dotenv().ok(); //loaded env file


    //securing the discord token
    let token = env::var("DISCORD_TOKEN").expect("missing discord token in .env file");

    //connecting to the database
    let database_url = env::var("DATABASE_URL").expect("Missing database url in env file");
    let db_pool = SqlitePool::connect(&database_url).await.expect("Failed to connect to database");

    //framework

    let framework = poise::Framework::builder()
    
    //configuration options
    .options(poise::FrameworkOptions {
        commands: vec![ping(), quote()], //register the ping command
        prefix_options: poise::PrefixFrameworkOptions{
            prefix: Some("!".into()), //prefix is !
            ..Default::default()
        },
        ..Default::default()
    })


    //setup function that runs once on startup
    .setup(move |_ctx, _ready, _framework| {

        Box::pin(async move {
            //load quotes from file
            let quotes_data = fs::read_to_string("quotes.json").unwrap_or("[]".to_string());
            let quotes: Vec<Quote> = serde_json::from_str(&quotes_data).expect("Error parsin quotes.json");


            //load episodes from files
            let episodes_data = fs::read_to_string("episodes.json").unwrap_or("[]".to_string());
            let episodes: Vec<Episode> = serde_json::from_str(&episodes_data).expect("Error parsing episodes.json");

            //creating Data Struct

            Ok( Data {

                database: db_pool,
                quotes,
                episodes,
                trivia_questions: vec![],
                current_trivia: Mutex::new(None),


            })

        })
    })


    //building
    .build();


    //creating client and running the bot

    let intents = serenity::GatewayIntents::non_privileged()
    | serenity::GatewayIntents::MESSAGE_CONTENT; //permissions

    let client = serenity::ClientBuilder::new(token,intents).framework(framework).await;

    client.unwrap().start().await.unwrap(); //infinite loop so the bot stays alive
    

    Ok(())
}
