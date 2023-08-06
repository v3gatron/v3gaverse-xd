use maud::{html, Markup, DOCTYPE};

fn feed() -> Markup {
    html! {
    div {
    ul {
        li {
            span {
            }
            div {
                div."feed-iota" {
                    "I do give one iota..."
                }
            }
            span.number {
                span {
                    "Iota"
                }
                span {
                }
            }
        }
        li {
            div {
                span {
                }
                div.title {
                    "First Note"
                }
                div.info {
                    "This is the first note for v3gaverse"
                }
            }
            span.number {
                span {
                    "Note"
                }
                span {
                }
            }
        }
        li {
            div {
                span {
                }
                div.title {
                    "New book added! \"Altered Carbon\""
                }
                div.info {
                    "I can't wait to read this book, I've been putting it off for a while. Lets justs see how much text this will allow me to put in it. Just for good measure lets type a bit more...a bit more...a bit more. Wow I'm a broken record."
                }
                div.type {
                    "Review"
                }
            }
            span.number {
                span {
                    "Book"
                }
                span {
                }
            }
        }
    }
    }
    }
}

fn header() -> Markup {
    html! {
        div #header {
                    div #header-menu {
                        div #logo {
                            img src="resources/v3gaverse-logo.png" {
                            }
                        }
                        div #menu.container {
                            ul {
                                a href="" {
                                    li {
                                        "Books"
                                    }
                                }
                                a href="" {
                                    li {
                                        "Stacks"
                                    }
                                }
                                a href="" {
                                    li {
                                        "Where Am I?"
                                    }
                                }
                            }
                        }
                        div #stack-dump-menu.container {
                            ul {
                                li {
                                    "»"
                                }
                                li {
                                    a href="" {
                                        "Braindust*"
                                    }
                                }
                                li {
                                    a href="" {
                                        "Polistack"
                                    }
                                }
                                li {
                                    a href="" {
                                        "StephenKingWorld"
                                    }
                                }
                                li {
                                    a href="" {
                                        "Tech"
                                    }
                                }
                            }
                        }
                    }
                    div #header-space {
                        "header space"
                    }
                    div #header-account-menu {
                        "Account menu"
                    }
                }

    }
}

pub async fn homepage() -> Markup {
    html! {
            (DOCTYPE)
            html lang="en" {
            head {
                meta charset="UTF-8" {
                }
                link href="resources/css/style.css?version=2" rel="stylesheet" {
                }
                title {
                    "v3gaverse: home"
                }
            }
            body {
                div.wrapper {
                    (header())
                    div #main {
                        div #sidebar {
                            div.intro {
                                p {
                                    "Welcome to"
                                    span.orange {
                                        " v3gaverse"
                                    }
                                    " - a unique digital space that marries the nostalgia of personal websites from the 90s with the dynamism of modern microblogging! As you explore, you'll discover a vibrant, eclectic fusion of my passions, hobbies, and ideas, all curated to both organize my thoughts and share them with like-minded souls who appreciate this delightful blend."
                                }
                                p {
                                    "Here, I celebrate the simplicity of the web's earlier days while embracing the power of modern technology to connect, inspire, and engage. Picture it as a digital scrapbook, showcasing my interests with thoughtful design and a touch of whimsy. This website offers a platform for creative expression, deep dives into my favorite topics, and the chance to build a community of people who share these common interests."
                                }
                                p {
                                    "So, step back in time and dive into the future with me at [Your Website Name] – a place where the old and the new collide to create an unforgettable online experience. Let's rediscover the magic of the internet together, one page and one post at a time. Welcome, and enjoy the journey!"
                                }
                            }
                            div #"sidebar-book-list" {
                                "Book list here"
                            }
                        }
                        div #content {
                            div #"front-page-monthly-feed" {
                                (feed())
                                                        }
                            div #"right-bar" {
                                "Advertisements, more book stuff, current events, later...this section will be busier"
                            }
                        }
                    }
                }
            }
        }
    }
}
