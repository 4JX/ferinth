use ferinth::Ferinth;
use std::{fs::File, io::Write};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("API error occured")]
    FerinthError(#[from] ferinth::Error),
    #[error("IO error occured")]
    IOError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // First, let's initialise the API
    // You should replace `example` with your program's name
    let api = Ferinth::new("example");

    // Now, lets get the Sodium project
    // You can use the project ID, or the project slug
    // The project ID will never change but the project slug can change at anytime

    // Using the project slug
    let _sodium = api.get_project("sodium").await?;
    // Using the project ID
    let _sodium = api.get_project("AANobbMI").await?;

    // Now lets get the versions that the Sodium mod has
    let sodium_versions = api.list_versions("AANobbMI", None).await?;

    // The versions are sorted chronologically so the first element should be the latest one
    let latest_version = &sodium_versions[0];
    // And now we can get this version's mod JAR file, which is called a version file
    let version_file = &latest_version.files[0];

    // Then we can download this version file
    let contents = api.download_version_file(version_file).await?;
    // And next, let's open the file we want to write this to
    let mut mod_file = File::create("Sodium.jar")?;
    // And finally, we can write the contents to mod_file
    mod_file.write_all(&contents)?;

    // Now you can load the JAR file using a mod loader. To play Sodium, you need the Fabric Loader
    Ok(())
}
