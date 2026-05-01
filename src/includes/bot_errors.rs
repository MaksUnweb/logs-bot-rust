#![allow(dead_code)]
use thiserror::Error;
use log::error;

#[derive(Error, Debug)]
pub enum BotErrors {
    #[error("Database working error!")]
    DbErrorFatal(#[from] sqlx::error::Error)
}

impl BotErrors {
    fn log(&self) {
       match self {
        BotErrors::DbErrorFatal(e) => {
            error!("{}",e);
        }
       } 
    }
}
