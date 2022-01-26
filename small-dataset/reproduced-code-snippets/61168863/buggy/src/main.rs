use futures::TryFutureExt;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut content: Vec<u8> = vec![];
    let f = tokio::fs::File::open("myfilecontent")
        .and_then(|mut myfile| {
            myfile.read_buf(&mut content)
        });
    f.await;
    Ok(())
}
