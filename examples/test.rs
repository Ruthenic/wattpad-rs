use wattpad::Wattpad;

#[tokio::main]
async fn main() {
    let watt = Wattpad::new()
        .await
        .expect("Failed to create Wattpad client struct");
    let story = watt
        .get_story("336149308")
        .await
        .expect("Failed to load story");

    println!("{}", story.copyright);
}
