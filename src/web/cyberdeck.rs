use maud::{html, Markup, DOCTYPE};

// NOTE: I think for now you need to keep it very simple.  A menu to flip between lists of iotas, notes, stacks and books.  You can get fancier over time

pub async fn homepage() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
           head {
                meta charset="UTF-8" {
                }
                link href="resources/css/style.css" rel="stylesheet" {
                }
                title {
                    "v3gaverse: home"
                }
            }
            body {
                div #header {
                    "header"
                }

                p {
                    "Form here"
                }
            }
        }


    }
}
