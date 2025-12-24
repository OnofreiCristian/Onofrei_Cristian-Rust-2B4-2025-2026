use std::string;
use sqlx::SqlitePool;
use std::sync::Mutex; //Safe access across multiple threads


use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize, Clone)]

pub struct Quote {

    pub text: String,
    pub Doctor: String,
    pub Source: Option<String>,

}


#[derive(Debug, Deserialize, Clone)]
pub struct Episode {

    pub title: String,
    pub runtime: String,
    pub season: u32,
    pub episode_number: u32,
    pub description: String,

}

#[derive(Debug, Clone)]

pub struct TriviaQuestions{

    pub question: String,
    pub answer: String,
    pub category: String,


}


pub struct Data{

    pub database: SqlitePool, //database to remmeber points

    pub quotes: Vec<Quote>,
    pub episodes: Vec<Episode>,
    pub trivia_questions: Vec<TriviaQuestions>,

    pub current_trivia: Mutex<Option<TriviaQuestions>>,

}


