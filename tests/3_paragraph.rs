#[cfg(test)]
mod paragraph_tests {
    use wattpad::Wattpad;

    #[tokio::test]
    async fn content() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let story = watt
            .get_story("327425279")
            .await
            .expect("Failed to load story");

        let parts = story.get_parts().await.expect("Failed to get parts");

        let paras = parts[0]
            .get_paragraphs()
            .expect("Failed to get part 1's paragraphs");

        assert_eq!(paras[0].id, "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(paras[0].html, "<br>")
    }
}
