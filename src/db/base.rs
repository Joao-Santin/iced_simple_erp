use mongodb::{options::ClientOptions, Client};
use tokio;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client_uri = "mongodb+srv://vitor-santin:pimpimpim298@biplasdb.hgu6btk.mongodb.net/?retryWrites=true&w=majority";
    let options = ClientOptions::parse(client_uri).await?;

    let client = Client::with_options(options)?;
    println!("Conectado!");
    let db = client.database("BiplasDB");
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

