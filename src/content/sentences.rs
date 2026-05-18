use rand::seq::SliceRandom;

pub const QUOTES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog and the dog just sat there looking unimpressed.",
    "All that glitters is not gold, but it sure does catch the eye in a way that matters.",
    "To be or not to be, that is the question, whether tis nobler in the mind to suffer.",
    "The greatest glory in living lies not in never falling but in rising every time we fall.",
    "In the middle of every difficulty lies opportunity and the chance to grow beyond your limits.",
    "Life is what happens when you are busy making other plans for the future you never see.",
    "The way to get started is to quit talking and begin doing the actual work right now.",
    "If life were predictable it would cease to be life and be without flavor or surprise.",
    "If you look at what you have in life you will always have more than you think.",
    "If you set your goals ridiculously high and fail your failure will be a new starting point.",
    "You will face many defeats in life but never let yourself be defeated by what you face.",
    "The greatest glory in living lies not in never falling but in rising every single time.",
    "In the end it is not the years in your life that count but the life in your years.",
    "Never let the fear of striking out keep you from playing the game you love the most.",
    "Life is either a daring adventure or nothing at all and the choice belongs to you.",
    "Many of life's failures are people who did not realize how close they were to success.",
    "You have brains in your head and feet in your shoes and you can steer yourself any direction.",
    "If you want to live a happy life tie it to a goal not to people or things.",
    "Never let the fear of striking out keep you from playing the game you care about.",
    "Money and success do not change people they merely amplify what is already there inside.",
    "Your time is limited so do not waste it living someone else's life or chasing their dream.",
    "Not how long but how well you have lived is the main thing that matters in the end.",
];

pub fn random_quote() -> &'static str {
    let mut rng = rand::thread_rng();
    QUOTES.choose(&mut rng).copied().unwrap_or(QUOTES[0])
}
