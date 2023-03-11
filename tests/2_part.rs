#[cfg(test)]
mod parts_tests {
    use wattpad::{Part, Wattpad};

    #[tokio::test]
    async fn init() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");

        story.get_parts().await.expect("Failed to get parts");
    }

    // no way ids chonny jash reference
    #[tokio::test]
    async fn id() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");

        let parts = story.get_parts().await.expect("Failed to get parts");

        assert_eq!(parts[0].id, 1288785987);
    }

    #[tokio::test]
    async fn title() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");

        let parts = story.get_parts().await.expect("Failed to get parts");

        assert_eq!(parts[0].title, "Chapter 1 - Welcome to the Studio");
    }

    #[tokio::test]
    async fn from_id() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");

        let part = Part::from_id("1288785987".to_string(), &watt.client)
            .await
            .expect("Failed to get part from ID");
    }
}
