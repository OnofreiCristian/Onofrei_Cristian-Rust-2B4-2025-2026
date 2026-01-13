mod structs;

use poise::serenity_prelude as serenity; //this is used to access permissions and users / channels



use sqlx::SqlitePool; //database
//used to access database
use std::fs; //to read JSON files
use std::env; //to read enviroment files
use std::sync::Mutex; // to share trivia state
use dotenv::dotenv; // to load the .env file
use std::sync::Arc;

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

#[poise::command(prefix_command)]
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


#[poise::command(prefix_command)]
pub async fn ask(ctx: Context<'_>) -> Result<(),Error> {

    let data = ctx.data();

    //picking a random question

    let question_opt = {

        let mut rng = rand::rng();
        data.trivia_questions.choose(&mut rng).cloned()
    };

    if let Some(q) = question_opt {

        //setting the q as the active question

        {

        let mut state = data.current_trivia.lock().unwrap();

        *state = Some(q.clone());

        }

        ctx.say(format!("** Trivia Time!** \n**{}**", q.question)).await?;

    } 
    else {
        ctx.say("No trivia question loaded!").await?;
    }

    Ok(())


} 

async fn event_handler(

    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_,Data,Error>,
    data: &Data,


) -> Result<(), Error>
{

    if let serenity::FullEvent::Message { new_message } = event {


        //ignore bots
        if new_message.author.bot {
            return Ok(());
        }


        //check if theres an active trivia question 
        //using a scope { } to lock, check, and potentially clear the question automatically
        //this prevents two people from answering correctly at the exact same millisecond 

        let correct_answer_found = {

            let mut trivia_state = data.current_trivia.lock().unwrap();

            if let Some(active_question) = &*trivia_state {

                //check if the users message matches the answer

                if new_message.content.trim().to_lowercase() == active_question.answer.trim().to_lowercase() {

                    *trivia_state = None;
                    true

                }

                else {

                    false
                }
            }

            else {

                false

            }

        };

            if correct_answer_found {
        
        let user_id = new_message.author.id.to_string();
        let username = &new_message.author.name;

        sqlx::query!(

            "INSERT INTO users (user_id, points) VALUES (?, 1)
             ON CONFLICT(user_id) DO UPDATE SET points = points + 1",
             user_id
        ).execute(&data.database).await?;

        let response = format!("**Correct!** {} gets a point!", username);

        new_message.channel_id.say(&ctx.http,response).await?;


     }
    }


    //awarding points to winner


    Ok(())


}

async fn start_trivia_loop(

    http: Arc<serenity::Http>, //the tool to send messages
    questions: Arc<Vec<structs::TriviaQuestions>>, //the list of questions
    state: Arc<Mutex<Option<structs::TriviaQuestions>>>, // the active question to lock


) {

    let channel_id = serenity::ChannelId::new(977191248358150174);

    loop {

        //waiting for interval (30 seconds for testing)

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;


        //check the state
        let is_game_active = {

            let lock = state.lock().unwrap();
            lock.is_some()

        };

        //post new question if needed

        if is_game_active{

            continue;
        }

            //pick a random question

            let new_question = {

                let mut rng =rand::rng();

                questions.choose(&mut rng).cloned()

            };

            if let Some(q) = new_question {

                {
                let mut lock = state.lock().unwrap();

                *lock = Some(q.clone());
                
                }
                
                //send the message

            let msg = format!("** Random Trivia Time! ** \n **{}**", q.question);

                let _ = channel_id.say(&http,msg).await;
            }

        }
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
        commands: vec![ping(), quote(), doctor(), episode(), points(), ask()], //register the commands

        event_handler: |ctx, event, framework, data| {

            Box::pin(event_handler(ctx, event, framework, data))

        },

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
            let questions_vec: Vec<TriviaQuestions> = serde_json::from_str(&trivia_data).expect("Error parsin trivia.json file.");

            //wrapping data in Arc for sharing
            let trivia_questions = Arc::new(questions_vec);
            let current_trivia = Arc::new(Mutex::new(None));


            //spawning background task
            //we clone the arc pointers to give the background task its own handle to see the data

            let http_clone = _ctx.http.clone();
            let questions_clone = trivia_questions.clone();
            let state_clone = current_trivia.clone();

            tokio::spawn(async move {

                start_trivia_loop(http_clone, questions_clone, state_clone).await;

            });


            println!("Loaded {} trivia questions", trivia_questions.len());


            //creating Data Struct

            Ok( Data {

                database: db_pool,
                quotes,
                episodes,
                trivia_questions,
                current_trivia,


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
