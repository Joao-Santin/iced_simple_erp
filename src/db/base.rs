use mongodb::{options::ClientOptions, Client};
use tokio;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client_uri = "alguma coisa";
    let options = ClientOptions::parse(client_uri).await?;

    let client = Client::with_options(options)?;
    println!("Conectado!");
    let db = client.database("biplas");
    let collection = db.collection("my_collection");
    use mongodb::bson::doc;
    let doc = doc! {
        "nome": "joao",
        "idade": 30,
        "ativo": true,
    };
    collection.insert_one(doc).await?;
    println!("Doc inserido");
    Ok(())
}

