use actix_web::{App, HttpServer, web::{self, Json}, Result, HttpResponse};
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, Error};
use serde::{Deserialize, Serialize};
use s3::bucket::Bucket;
use s3::region::Region;
use s3::creds::Credentials;
use std::str;
use std::time::Instant;
use chrono::prelude::*;


#[derive(Debug, Deserialize)]
struct GreeneryID {
    greenery_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeneralInfo {
    greenery_id: String,
    name: String,
    phone: String,
    email: String,
    address: String,
}

async fn get_bucket(bucket: &str) -> Result<Bucket> {
    let credentials: Credentials = match Credentials::default().await {
    // let credentials: Credentials = match Credentials::new(Some(""), 
    //                                                       Some(""), 
    //                                                       None, None, None).await {
        Ok(credentials) => credentials,
        Err(e) => {
            println!("{}: Error loading credentials: {}", Utc::now(), e);
            return Err(ErrorInternalServerError("Error loading credentials"));
        },
    };
    let region: Region = match "us-west-2".parse() {
        Ok(region) => region,
        Err(e) => {
            println!("{}: Error parsing region name: {}", Utc::now(), e);
            return Err(ErrorInternalServerError("Error parsing region name"))
        },
    };
    match Bucket::new(bucket, region, credentials) {
        Ok(bucket) => Ok(bucket),
        Err(e) => {
            println!("{}: Error creating datastore object: {}", Utc::now(), e);
            return Err(ErrorInternalServerError("Error creating datastore object"))
        },
    }
}

async fn get_string_content_from_bucket(bucket: Bucket, key: &str) -> Result<String> {
    match bucket
    .get_object(key)
    .await {
        Ok(result) => {
            let (content, code) = result;
            match str::from_utf8(&content) {
                Ok(string_content) => {
                    if code == 404 {
                        println!("{}: Couldn't find object in datastore. code: {}, message: {:?}", Utc::now(), code, string_content);
                        return Err(ErrorNotFound("Couldn't find object in datastore"))
                    } else if code != 200 {
                        println!("{}: Error getting object from datastore. code: {}, message: {:?}", Utc::now(), code, string_content);
                        return Err(ErrorInternalServerError("Error getting object from datastore"))
                    }
                    Ok(string_content.to_owned())
                },
                Err(e) => {
                    println!("{}: Error parsing object content to string: {}", Utc::now(), e);
                    Err(ErrorInternalServerError("Error parsing object content to string"))
                },
            }
        },
        Err(e) => {
            println!("{}: Error reading content from datastore: {}", Utc::now(), e);
            return Err(ErrorInternalServerError("Error reading content from datastore"))
        },
    }
}

async fn read_general(greenery_id_json: Json<GreeneryID>) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    println!("{}: /readGeneral starting", Utc::now());

    let bucket = get_bucket("greenery-datastore").await?;
    let mut key = "/general/".to_owned();
    key.push_str(&greenery_id_json.greenery_id);
    key.push_str(".json");
    let json_string = get_string_content_from_bucket(bucket, &key).await?;

    let response = HttpResponse::Ok()
        .content_type("application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(json_string);

    println!("{}: /readGeneral finished, took: {}ms", Utc::now(), now.elapsed().as_millis());
    Ok(response)
}

async fn write_string_content_to_bucket(bucket: Bucket, key: &str, content: &str) -> Result<()> {
    match bucket.put_object(key, content.as_bytes(), "application/json").await {
        Ok(result) => {
            let (content, code) = result;
            if code != 200 {
                match str::from_utf8(&content) {
                    Ok(string_content) => {
                        println!("{}: Error creating content in datastore. code: {}, message: {:?}", Utc::now(), code, string_content);
                        return Err(ErrorInternalServerError("Error creating content in datastore"))
                    },
                    Err(e) => {
                        println!("{}: Error creating content in datastore. code: {}, Failed to parse error message: {:?}", Utc::now(), code, e);
                        return Err(ErrorInternalServerError("Error creating content in datastore"))
                    }
                }

            } else {
                Ok(())
            }
        },
        Err(e) => {
            println!("{}: Error creating content in datastore: {}", Utc::now(), e);
            return Err(ErrorInternalServerError("Error creating content in datastore"))
        }
    }
}

async fn create_general(general_info_json: Json<GeneralInfo>) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    println!("{}: /createGeneral starting", Utc::now());

    let bucket = get_bucket("greenery-datastore").await?;
    let mut key = "/general/".to_owned();
    key.push_str(&general_info_json.greenery_id);
    key.push_str(".json");

    let string_json = serde_json::to_string(&general_info_json.into_inner())?;
    write_string_content_to_bucket(bucket, &key, &string_json).await?;

    let response = HttpResponse::Ok()
    .content_type("application/json")
    .header("Access-Control-Allow-Origin", "*")
    .body("");

    println!("{}: /createGeneral finished, took: {}ms", Utc::now(), now.elapsed().as_millis());
    Ok(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let address = String::from("0.0.0.0:5000");
    println!("{}: Listening on: {}", Utc::now(), address);
    HttpServer::new(move || {
         App::new()
            .route("/readGeneral", web::post().to(read_general))
            .route("/createGeneral", web::post().to(create_general))
    })
    .bind(address)?
    .run()
    .await
}