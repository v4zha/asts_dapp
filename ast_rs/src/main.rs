// use ethers::contract::Contract;
// use ethers::prelude::{rand::thread_rng, *};
use ethers::prelude::rand::thread_rng;
use ethers::prelude::*;
use std::error::Error as Err;
use std::sync::Arc;
mod std_events;
use std_events::{EventDetails, StdEvent};
// abigen!(StdEvent, "./contracts/ASTEvent.json");

#[allow(dead_code)]
enum Department {
    Mech,
    EEE,
    Civil,
    EC,
    CS,
    IT,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Err>> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let wallet = LocalWallet::new(&mut thread_rng());
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    let contract_address = "0xc2502922724e7Ed844d06A029672355534f448d2".parse::<Address>()?;
    let std_events = Arc::new(StdEvent::new(contract_address, Arc::clone(&client)));

    let event_counter: u64 = std_events.token_counter().call().await?.as_u64();
    let event = std_events.events();
    let mut event_stream = event.stream().await?;
    println!("Waiting for ASTEvent events : ) ");
    while let Some(Ok(event)) = event_stream.next().await {
        println!("Emit Event : {:?}", event);
        for i in 0..event_counter + 1 {
            let std_event: EventDetails =
                std_events.get_ast_event_data(U256::from(i)).call().await?;
            println!("Event {} : {:?} ", i + 1, std_event);
        }
    }

    Ok(())
}
