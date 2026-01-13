use std::sync::{Arc, Mutex};

use sqlx::SqlitePool;


use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]

pub struct Quote {

    pub text: String,
    pub doctor: String,
    pub source: Option<String>,

}


#[derive(Debug, Deserialize, Clone)]
pub struct Episode {

    pub name: String,
    pub runtime: u32,
    pub season: u32,
    pub number: u32,
    pub summary: String,

}

#[derive(Debug, Clone, Deserialize)]

pub struct TriviaQuestions{

    pub question: String,
    pub answer: String,


}


pub struct Data{

    pub database: SqlitePool, //database to remmeber points

    pub quotes: Vec<Quote>,
    pub episodes: Vec<Episode>,

    pub trivia_questions: Arc<Vec<TriviaQuestions>>,

    pub current_trivia: Arc<Mutex<Option<TriviaQuestions>>>,

}


