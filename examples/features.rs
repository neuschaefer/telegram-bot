extern crate rustc_serialize;
extern crate telegram_bot;

use telegram_bot::*;
use std::env;
use rustc_serialize::json;

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
            let name = m.from.first_name + &*m.from.last_name
                .map_or("".to_string(), |mut n| { n.insert(0, ' '); n });
            let chat_id = m.chat.id();

            // Match message type
            match m.msg {
                MessageType::Text(t) => {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);

                    // Define one time response keyboard
                    let keyboard = ReplyKeyboardMarkup {
                        keyboard: vec![vec![t],
                                       vec!["Yes".into(), "No".into()]],
                       one_time_keyboard: Some(true),
                        .. Default::default()
                    };

                    // Reply with custom Keyboard
                    try!(bot.send_message(
                        chat_id,
                        format!("Hi, {}!", name),
                        None, None, Some(keyboard.into())));

                },
                MessageType::Location(loc) => {
                    // Print event
                    println!("<{}> is here: {}", name,
                        json::encode(&loc).unwrap());

                    // Send chat action (this is useless here, it's just for
                    // demonstration purposes)
                    try!(bot.send_chat_action(chat_id, ChatAction::Typing));

                    // Calculate and send the location on the other side of the
                    // earth.
                    let lat = -loc.latitude;
                    let lng = if loc.longitude > 0.0 {
                        loc.longitude - 180.0
                    } else {
                        loc.longitude + 180.0
                    };

                    try!(bot.send_location(chat_id, lat, lng, None, None));
                },
                MessageType::Contact(c) => {
                    // Print event
                    println!("<{}> send a contact: {}", name,
                        json::encode(&c).unwrap());

                    // Just forward the contact back to the sender...
                    try!(bot.forward_message(chat_id, chat_id, m.message_id));
                }
                _ => {}
            }

        }
        Ok(())
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
