extern crate telegram_bot;

use telegram_bot::*;
use std::env;

fn main() {
    // Fetch environment variable with bot token
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) =>
            panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing! {}", e),
    };

    // Create bot, test simple API call and print bot information
    let mut bot = Bot::new(token);
    println!("getMe: {:?}", bot.get_me());

    // Fetch new updates via long poll method
    let res = bot.long_poll(None, |bot, u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            let name = m.from.first_name;

            // Match message type
            match m.msg {
                MessageType::Text(t) => {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);

                    if t == "/exit" {
                        return Err(Error::UserInterrupt);
                    }

                    // Answer message with "Hi"
                    try!(bot.send_message(
                        m.chat.id(),
                        format!("Hi, {}! You just wrote '{}'", name, t),
                        None, None, None));
                },
                _ => {}
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(())
    });

    // When the method `long_poll` returns, its due to an error. Check it here.
    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
