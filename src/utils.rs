use rand::Rng;

pub fn get_love_sentence() -> String {
    let sentences: Vec<&str> = vec![
        "You had me at 'let's order Indian food'.",
        "I love you more than chocolate...",
        "You're my favorite notification.",
        "You must be a magician, because every time I look at you, everyone else disappears.",
        "You're the nutella to my bread.",
        "If kisses were snowflakes, I'd send you a blizzard.",
        "You're like Wi-Fi — I feel lost without you.",
        "I'm much more me when I'm with you.",
        "You make my heart do a little cha-cha.",
        "You're proof that love is the best kind of magic.",
        "You had me at your first awkward smile.",
        "You're my favorite daydream.",
        "If loving you is wrong, I don't want to be right.",
        "You're the plot twist I didn't know I needed.",
        "I love you more than Netflix on a rainy day.",
        "You're my human diary and my happy place.",
        "You're my person, in every universe.",
        "I still get butterflies — and sometimes they tap dance.",
        "You're my 'can't wait to tell you' person.",
        "You're the sweetest potato of them all",
        "You're my favorite hello and hardest goodbye.",
        "You make me believe in happy endings.",
        "You're the song stuck in my head… forever.",
        "You make my heart race faster than free dessert.",
        "You're my partner in crime and in cuddles.",
        "I'd pause my coding for you — and that's love.",
        "You're the sparkle in my mundane Tuesday.",
        "With you, even doing nothing feels like everything.",
        "You're the best chapter in my story.",
        "I love you more than I hate early mornings.",
    ];
    let mut rng = rand::rng();
    let l = sentences.len();
    let idx = rng.random_range(0..l);
    sentences[idx].to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_love_sentence() {
        let sentences: Vec<String> = vec![
            "You had me at 'let's order Indian food'.".to_string(),
            "I love you more than chocolate...".to_string(),
            "You're my favorite notification.".to_string(),
            "You must be a magician, because every time I look at you, everyone else disappears."
                .to_string(),
            "You're the nutella to my bread.".to_string(),
            "If kisses were snowflakes, I'd send you a blizzard.".to_string(),
            "You're like Wi-Fi — I feel lost without you.".to_string(),
            "I'm much more me when I'm with you.".to_string(),
            "You make my heart do a little cha-cha.".to_string(),
            "You're proof that love is the best kind of magic.".to_string(),
            "You had me at your first awkward smile.".to_string(),
            "You're my favorite daydream.".to_string(),
            "If loving you is wrong, I don't want to be right.".to_string(),
            "You're the plot twist I didn't know I needed.".to_string(),
            "I love you more than Netflix on a rainy day.".to_string(),
            "You're my human diary and my happy place.".to_string(),
            "You're my person, in every universe.".to_string(),
            "I still get butterflies — and sometimes they tap dance.".to_string(),
            "You're my 'can't wait to tell you' person.".to_string(),
            "You're the sweetest potato of them all".to_string(),
            "You're my favorite hello and hardest goodbye.".to_string(),
            "You make me believe in happy endings.".to_string(),
            "You're the song stuck in my head… forever.".to_string(),
            "You make my heart race faster than free dessert.".to_string(),
            "You're my partner in crime and in cuddles.".to_string(),
            "I'd pause my coding for you — and that's love.".to_string(),
            "You're the sparkle in my mundane Tuesday.".to_string(),
            "With you, even doing nothing feels like everything.".to_string(),
            "You're the best chapter in my story.".to_string(),
            "I love you more than I hate early mornings.".to_string(),
        ];
        let sentence = get_love_sentence();
        assert!(&sentences.contains(&sentence));
    }
}
