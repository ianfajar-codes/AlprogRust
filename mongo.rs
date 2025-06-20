use dotenv::dotenv;
use std::env;
use crate::model::SensorData;
use mongodb::{Client, options::ClientOptions, bson::doc, Collection};
use mongodb::error::Error;
use futures::stream::TryStreamExt;

pub async fn get_client() -> Result<Client, Error> {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI not set");
    let options = ClientOptions::parse(uri).await?;
    Client::with_options(options)
}

pub async fn fetch_data(client: &Client) -> Result<Vec<SensorData>, Error> {
    let db = client.database("sensor_gas");
    let coll: Collection<SensorData> = db.collection("data_sensor");

    let mut cursor = coll.find(None, None).await?;
    let mut results = Vec::new();

    while let Some(doc) = cursor.try_next().await? {
        results.push(doc);
    }

    Ok(results)
}

pub async fn insert_data(client: &Client, value: f64) -> Result<(), Error> {
    let db = client.database("sensor_gas");
    let coll: Collection<SensorData> = db.collection("data_sensor");

    let new_data = SensorData {
        timestamp: bson::DateTime::now(),
        value,
};


    coll.insert_one(new_data, None).await?;
    Ok(())
}
