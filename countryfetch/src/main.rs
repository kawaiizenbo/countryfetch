use crate::country_format::format_country;
use std::{io::Read, path::PathBuf};

use countryfetch::{Country, Location};

mod country_format;
mod generated;

async fn get_data() -> Result<(Location, Country), Box<dyn std::error::Error>> {
    let ip = public_ip::addr().await.unwrap();
    dbg!(ip);

    let client = reqwest::Client::new();

    let location = client.get(format!(
        "http://ip-api.com/json/{ip}?fields=status,message,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,query"
    )).send().await?.json::<Location>().await?;

    let country = client
        .get(format!(
            "https://restcountries.com/v3.1/alpha/{}",
            location.country_code
        ))
        .send()
        .await?
        .json::<Vec<Country>>()
        .await?
        .into_iter()
        .next()
        .expect("Returns a single country from querying for a Country Code, when there is a valid country code (which would have failed earlier)");

    Ok((location, country))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (location, country) = get_data().await.unwrap();

    let mut country_json =
        std::fs::File::open(PathBuf::from("../../xtask/countries.json")).unwrap();

    let mut buf = String::new();

    country_json.read_to_string(&mut buf).unwrap();

    let countries = serde_json::de::from_str::<Vec<Country>>(&buf).unwrap();

    for country in countries {
        let gen_country = generated::Country::from_country_code(&country.country_code3).unwrap();
        let country = format_country(gen_country, Some(&country), None);

        println!("{country}");
    }

    // let country_cached_data = generated::Country::from_country_code(&country.country_code3)
    //     .expect("All countries have been cached");

    // dbg!(location, country);

    Ok(())
}
