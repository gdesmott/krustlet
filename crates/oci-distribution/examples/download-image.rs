use anyhow::Result;

use oci_distribution::{manifest, secrets::RegistryAuth, Client};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();

    let reference = std::env::args().nth(1).expect("missing image name");
    let reference = reference.parse()?;
    dbg!(&reference);

    let mut client = Client::default();
    let img = client
        .pull(
            &reference,
            &RegistryAuth::Anonymous,
            vec![
                manifest::WASM_LAYER_MEDIA_TYPE,
                manifest::IMAGE_MANIFEST_MEDIA_TYPE,
                manifest::IMAGE_DOCKER_LAYER_GZIP_MEDIA_TYPE,
            ],
        )
        .await?;

    println!("Downloaded {}", img.digest());

    Ok(())
}
