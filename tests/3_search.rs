#[cfg(test)]
mod text_search_tests {
    use wattpad::{SearchSort, SearchType, Wattpad};

    #[tokio::test]
    async fn specific_search() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let search = watt
            .search(
                "Inky Desires [Bendy x Reader]",
                SearchType::Text,
                SearchSort::Hot,
                30,
            )
            .await
            .expect("Failed to create Search");

        let results = search.page(0).await.expect("Failed to get search results");

        assert_eq!(results[0].id, "327425279")
    }

    #[tokio::test]
    async fn broad_search() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let search = watt
            .search("huggy wuggy smut", SearchType::Text, SearchSort::Hot, 30)
            .await
            .expect("Failed to create Search");

        let results = search.page(0).await.expect("Failed to get search results");

        assert!(results.len() == 30)
    }
}

#[cfg(test)]
mod tag_search_tests {
    use wattpad::{SearchSort, SearchType, Wattpad};

    #[tokio::test]
    async fn specific_search() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let search = watt
            .search("bluesourpachkid", SearchType::Tag, SearchSort::Hot, 30)
            .await
            .expect("Failed to create Search");

        let results = search.page(0).await.expect("Failed to get search results");

        assert_eq!(results[0].id, "290528000")
    }

    #[tokio::test]
    async fn broad_search() {
        let watt = Wattpad::new()
            .await
            .expect("Failed to create Wattpad client struct");
        let search = watt
            .search("bendyxreader", SearchType::Tag, SearchSort::Hot, 30)
            .await
            .expect("Failed to create Search");

        let results = search.page(0).await.expect("Failed to get search results");

        assert!(results.len() == 30);

        let results = search.page(0).await.expect("Failed to get search results");

        assert!(results.len() == 30)
    }
}
