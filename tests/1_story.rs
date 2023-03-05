#[cfg(test)]
mod story_tests {
    use wattpad::Wattpad;

    #[tokio::test]
    async fn tags() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");

        assert!(story.tags.contains(&"bendyxreader".to_string()))
    }

    #[tokio::test]
    async fn metadata() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");
        assert_eq!(story.title, "Inky Desires [Bendy X Reader]")
    }

    #[tokio::test]
    async fn author() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");

        story
            .get_author()
            .await
            .expect("Failed to get author from story");
    }
}
