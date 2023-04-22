use itertools::Itertools;

fn main() {
    let auth = match std::env::var("LINE_NOTIFY_TOKEN") {
        Ok(mut token) => {
            token.insert_str(0, "Bearer ");
            token
        }
        Err(e) => {
            eprintln!("Failed to get token from LINE_NOTIFY_TOKEN: {e}");
            return;
        }
    };

    let message = match std::io::read_to_string(std::io::stdin()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read stdin: {e}");
            return;
        }
    };

    // TODO: The strict limit is "1000 UTF-16 unit" but I'm too lazy to implement it.
    // I suppose the UTF-8 chars limit below is not very bad for now.
    const MAX_MESSAGE_LENGTH: usize = 1000;

    const API_ENDPOINT: &str = "https://notify-api.line.me/api/notify/";

    // TODO: properly handle the input as UTF-16 units
    for chunk in &message.chars().chunks(MAX_MESSAGE_LENGTH) {
        let chunk = chunk.collect_vec();
        let len = chunk.len();
        let chunk = chunk.into_iter().collect::<String>();
        match ureq::post(API_ENDPOINT)
            .set("Authorization", &auth)
            .send_form(&[("message", &chunk)])
        {
            Ok(_) => {
                println!("Sent {len} chars")
            }
            Err(e) => {
                eprintln!("Failed to send request: {e}");
                return;
            }
        }
    }
}
