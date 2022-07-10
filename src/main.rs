pub mod art;

use std::fmt;
use std::iter::repeat;

struct Art<'a> {
    content: &'a str,
}

impl fmt::Display for Art<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding: String = repeat(' ').take(f.width().unwrap_or(0)).collect();

        self.content
            .lines()
            .map(|line| write!(f, "{padding}{line}\n"))
            .collect()
    }
}

struct MessageBox<'a> {
    content: &'a str,
}

impl fmt::Display for MessageBox<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.content.len() + 4;
        let ceiling: String = repeat('-').take(width).collect();

        vec![&ceiling, &format!("| {} |", self.content), &ceiling]
            .iter()
            .map(|s| write!(f, "{s}\n"))
            .collect()
    }
}

struct Message<'a> {
    message: MessageBox<'a>,
    art: Art<'a>,
}

impl Message<'_> {
    fn render(&self) -> String {
        let mut render = String::new();

        let message_render = self.message.to_string();
        render.push_str(&message_render);

        let message_middle = message_render.len() / message_render.lines().count() / 3;
        let steps = 4;
        for padding in message_middle..=(message_middle + steps) {
            let padding: String = repeat(' ').take(padding).collect();
            render.push_str(&format!("{padding}\\\n"));
        }

        let pad = message_middle;
        render.push_str(&pad_string(&self.art.to_string(), pad));

        render
    }

    fn display(&self) {
        println!("{}", self.render());
    }
}

fn pad_string(s: &str, amount: usize) -> String {
    let padding: String = repeat(' ').take(amount).collect();
    s.lines().map(|l| format!("{padding}{l}\n")).collect::<String>().trim_end().to_owned()
}

fn main() {
    let message = Message {
        message: MessageBox {
            content: "WHAT THE FUCK IS GOING ONNN!????",
        },
        art: Art {
            content: art::CRAB,
        },
    };

    message.display();
}
