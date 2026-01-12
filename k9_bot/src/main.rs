mod structs;

use poise::serenity_prelude as serenity; //this is used to access permissions and users / channels


use sqlx::Sqlite; //database
use sqlx::SqlitePool; //used to access database
use std::fs; //to read JSON files
use std::env; //to read enviroment files
use std::sync::Mutex; // to share trivia state
use dotenv::dotenv; // to load the .env file

use structs::{Data,Quote,Episode};

//using this to choose random quotes
use rand::rng;
use rand::prelude::IndexedRandom;

use crate::structs::TriviaQuestions;


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
            quote.doctor,
            quote.source.as_ref().map(|s| format!(" ({})", s)).unwrap_or_default() 
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

#[poise::command(prefix_command)]
pub async fn doctor(

    ctx: Context<'_>,
    #[description = "Which Doctor do you want to see?"]
    number: u32, //argument


) -> Result<(),Error>{


    let filename = format!("images/{}.jpg", number); //construit numele fisierului


    //this checks if the file exists in the first place
    if !std::path::Path::new(&filename).exists() {

        ctx.say(format!("I couldn't find an image of Doctor #{} in the archives.",number)).await?;

        return Ok(());
    }

    let attachment = serenity::CreateAttachment::path(&filename).await;


    //sending message if we found file or error otherwise
    match attachment {

        Ok(file) => {

            ctx.send(poise::CreateReply::default().attachment(file)).await?;

        },

        Err(e) => {

            println!("Error loading file {}", e);

            ctx.say("Error: File load error!").await?;

        }

    }


    Ok(())

}


#[poise::command(prefix_command)]
pub async fn episode(

    ctx: Context <'_>,
    #[description = "The name of the episode"]
    #[rest] //captures the entire message as one string
    query: String
    
) -> Result<(), Error> {

    let data = ctx.data();

    //searching for the episode 
    //we make both strings lowercase - Blink = blink

    let episode = data.episodes.iter().find(|e|
        
        e.name.to_lowercase().contains(&query.to_lowercase())

    );


    //handling the result

    match episode {

        Some(ep) => {

                let embed = serenity::CreateEmbed::new()
                .title(&ep.name)
                .description(&ep.summary)
                //picking color
                .color(0x003b6f)
                .field("Season", ep.season.to_string(), true) //true = inline
                .field("Episode", ep.number.to_string(), true)
                .field("Runtime", ep.runtime.to_string(), true);

            //send the message with the embed
            //we use create reply to wrap the embed

            ctx.send(poise::CreateReply::default().embed(embed)).await?;

        },

        None => {

            ctx.say(format!("Couldn't find any episode matching \"{}\".", query)).await?;

        }

    }


    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn points(ctx: Context<'_>) -> Result<(), Error> {


    let data = ctx.data();

    //accessing the database;

    let rows = sqlx::query!(

        "SELECT user_id, points FROM users ORDER BY points DESC LIMIT 10"

    )

    .fetch_all(&data.database)
    .await?;


    //formatting the leaderboard on discord

    let mut response = String::from("** Time Lord Leaderboard **\n");


    if rows.is_empty() {

        response.push_str("No points awarded yet. Be the first!");
    }
    else {

        for(i, row) in rows.iter().enumerate() {

            //use <@user_id> so discord automatically highlights name

            let user_id = row.user_id.clone().unwrap_or_default();
            let score = row.points.unwrap_or(0);


            response.push_str(&format!("{}. <@{}> - **{}** points\n",i+1,user_id,score));
        }

    }

    ctx.say(response).await?;

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
        commands: vec![ping(), quote(), doctor(), episode(), points()], //register the ping command
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


            //load trivia from file
            let trivia_data = fs::read_to_string("trivia.json").unwrap_or("[]".to_string());
            let trivia_questions: Vec<TriviaQuestions> = serde_json::from_str(&trivia_data).expect("Error parsin trivia.json file.");

            println!("Loaded {} trivia questions", trivia_questions.len());


            //creating Data Struct

            Ok( Data {

                database: db_pool,
                quotes,
                episodes,
                trivia_questions,
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
