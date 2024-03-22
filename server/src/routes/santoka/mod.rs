use maud::{html, Markup, Render};
use shared::controllers::show_hide::ShowHide;
use shared::controllers::show_if_scrolled::show_if_scrolled;
use shared::route::Route;
use shared::santoka_haiku_2023_06_26::*;

pub mod non_preview_poems;

use crate::assets::ASSETS;
use crate::components::*;
use crate::css_class_groups::*;
use crate::extensions::StrExtension;

pub fn page() -> Markup {
    Layout::new(
        "Taneda Santōka",
        "Taneda Santōka | 種田山頭火",
        html! {
            (floating_nav())
            main
                class="relative p-8 lg:p-8 flex flex-col gap-8 lg:gap-32 w-full max-w-screen-2xl" {
                (hero_section())
                (poetry_section())
            }
        },
    )
    .render()
}

fn floating_nav() -> Markup {
    html! {
        div
            class={"
                cover-nav
                z-20
                absolute
                -top-32 left-0
                w-screen
                p-32 "
                (bg_background())} {}
        nav
            class={"
                fixed top-0 z-10
                {horizontal_center_fixed()}
                w-full max-w-screen-2xl 
                pointer-events-none
                px-8 lg:px-8
                text-base lg:text-2xl tracking-wide "
                (show_if_scrolled())} {

            div
                // We use this extra div to prevent text from showing up once it has scrolled
                // up past the nav.
                class={"
                    vertical-spacer
                    pt-4 lg:pt-8
                    lg:w-1/4
                    lg:rounded-b-3xl
                    shadow-neutral-50 dark:shadow-neutral-900 "
                    (bg_background())
                }
                style="box-shadow: var(--tw-shadow-color) 0px 0px 0 2px;" {

                div
                    class="
                        logo-and-links 
                        bg-[linear-gradient(90deg,_#e6edee,_#98b7ca)]
                        dark:bg-[linear-gradient(90deg,#5b6f76,#212123)]
                        overflow-hidden rounded-3xl 
                        p-4
                        w-full
                        flex flex-row gap-4 justify-between 
                        pointer-events-auto
                        select-none" {
                    (nav_logo())
                    (nav_links())
                }
            }
        }
    }
}

pub fn nav_logo() -> Markup {
    Link::new()
        .href(Route::Home)
        .without_default_classes()
        .class(
            "
            logo
            block rounded-full
            w-6 lg:w-8 h-6 lg:h-8
            cursor-pointer
            "
            .plus(bg_background()),
        )
        .render()
}

pub fn nav_links() -> Markup {
    html! {
       div
           class="
                links
                flex flex-row gap-4
                font-light lg:font-normal
                text-neutral-100 dark:text-neutral-300" {
           (Link::new()
               .href(Route::Home)
               .class("tracking-wide")
               .slot("about"))
           (Link::new()
               .href(Route::Home)
               .class("tracking-wide")
               .slot("data + code"))
       }
    }
}

fn hero_section() -> Markup {
    html! {
       section
           id="hero"
           class="
                flex flex-col justify-start items-start dark:justify-end
                overflow-hidden rounded-3xl 
                relative
                w-full h-[384px] lg:h-[720px] max-h-[calc(100vh-4rem)]
                selection:bg-neutral-700/75 dark:selection:bg-neutral-500/75
                text-neutral-100 dark:text-neutral-300 tracking-wider
                z-20" {
           nav
               class="
                    logo-and-links absolute p-4 lg:p-8 top-0 left-0 w-full 
                    flex flex-row gap-4 justify-between 
                    text-base lg:text-2xl tracking-wider text-neutral-100 dark:text-neutral-200
                    z-10" {
               (nav_logo())
               (nav_links())
           }

           (hero_image())

           div
               class="absolute bottom-0 left-0 flex flex-col p-4 lg:p-8" {
               span
                   id="name-in-kanji"
                   class="text-4xl lg:text-7xl font-semibold tracking-wide whitespace-nowrap" {
                   "種田山頭火"
               }
               span
                   id="name-in-romaji"
                   class="text-xl lg:text-4xl font-normal tracking-wide whitespace-nowrap" {
                   "Taneda Santōka"
               }
               span
                   id="birth-and-death"
                   class="text-base lg:text-3xl font-normal tracking-wide whitespace-nowrap" {
                   "1882 – 1940"
               }
           }
       }
    }
}

fn poetry_section() -> Markup {
    html!(
        section id="poems" class="flex flex-col tracking-wide" {
            @for publication in DATABASE.publications.iter() {
                (poems_and_publication(publication))
            }
        }
    )
}

fn poems_and_publication(publication: &'static Publication) -> Markup {
    let show_hide = ShowHide::new();

    html!(
        div
            class={ "poems-and-publication flex flex-col lg:flex-row gap-10 lg:gap-12 " (show_hide.container())} {
            (publication_container(publication, &show_hide))
            (poems_in_publication(publication, &show_hide))
        }
    )
}

fn publication_container(publication: &'static Publication, show_hide: &ShowHide) -> Markup {
    let translator = DATABASE.translator(publication.translator_id);

    // a { class="underline decoration-1 underline-offset-4 cursor-pointer",
    //     "hide"
    // }
    html!(
        // self-start is necessary to make sticky work.
        div
            class={"
                publication
                sticky top-[4.5rem] lg:top-32 self-start
                py-4 lg:pt-0 lg:pb-32
                flex flex-col gap-2
                items-start lg:items-end
                w-full lg:w-1/4 
                lg:text-right
                text-lg lg:text-3xl
                text-neutral-400 dark:text-neutral-400 
                border-b border-neutral-200 dark:border-neutral-700 lg:border-0 "
                (bg_background())} {
            span class="translator font-light lg:font-extralight" {
                (translator.name)
            }
            span class="publication-name font-light lg:font-thin italic" {
                (publication.name)
            }
            span class="publication-year font-light lg:font-thin" {
                (publication.year_or_unknown()) " • "
                (Link::new()
                    .class(show_hide.toggle())
                    .slot(html! {
                        span class=(show_hide.show_by_default()) { "hide" }
                        span class=(show_hide.hide_by_default()) { "show" }
                    }))
            }
        }
    )
}

fn poems_in_publication(publication: &'static Publication, show_hide: &ShowHide) -> Markup {
    // We use padding-bottom instead of gap on the parent because of the way it affects the publication's
    // sticky positioning.
    html! {
        div
            class="
                poems-in-publication
                lg:w-1/2" {
            (visible_poems_in_publication(publication, show_hide))
            (hidden_poems_in_publication(show_hide))
            div class="scroll-anchor-when-hiding-publication pb-16 lg:pb-32" {}
        }
    }
}

fn visible_poems_in_publication(publication: &'static Publication, show_hide: &ShowHide) -> Markup {
    let show_by_default_class = show_hide.show_by_default();

    let gap = if publication.has_japanese_text() {
        "gap-8 lg:gap-20"
    } else {
        "gap-8 lg:gap-16"
    };

    html! {
        div
            class={"
                visible-poems-in-publication
                flex flex-col
                text-base lg:text-3xl "
                (gap)
                " "
                (show_by_default_class)} {
            @for preview_poem in publication.preview_poems() {
                (poem(preview_poem))
            }

            (load_more_poems(publication))
        }
    }
}

fn load_more_poems(publication: &'static Publication) -> Markup {
    if !publication.has_non_preview_poems() {
        return html! {};
    }

    let replace_class = shared::controllers::replace::replace(Route::NonPreviewPoems {
        publication_id: publication.id,
    });

    html!(
        // bg-neutral-200 dark:bg-neutral-600
        span
            class={"
                text-neutral-400 dark:text-neutral-400
                flex flex-row gap-2 "
                (replace_class)} {

            "⨀"

            button
                class={"
                    tracking-wider
                    whitespace-nowrap
                    font-thin
                    w-min "
                    (link_classes())} {
                "load more"
            }
        }
    )
}

fn hidden_poems_in_publication(show_hide: &ShowHide) -> Markup {
    html! {
        div
            class={"
                hidden-poems
                font-light lg:font-extralight
                text-sm lg:text-2xl 
                text-neutral-400 dark:text-neutral-500
                p-8
                w-3/4
                border border-neutral-200 dark:border-neutral-700
                rounded-2xl
                cursor-pointer "
                (show_hide.hide_by_default())
                " "
                (show_hide.toggle())} {
            "Click to show."
        }
    }
}

pub fn poem(poem: &'static Poem) -> Markup {
    let english_text = poem.english_text.replace("—\n", "—");
    let english_classes =
        "poem-english-text font-light lg:font-extralight text-neutral-400 dark:text-neutral-400";

    match &poem.japanese_text {
        Some(japanese_text) => {
            let romaji_classes = if poem.japanese_text_is_romaji() {
                "italic font-light lg:font-extralight"
            } else {
                "font-light lg:font-thin"
            };

            html!(
                div class="poem flex flex-col gap-1 lg:gap-2 lowercase " {
                    span class=(english_classes) {
                        (english_text)
                    }
                    span
                        class={"
                            poem-japanese-text
                            text-neutral-300 dark:text-neutral-500 "
                            (romaji_classes)} {
                        (japanese_text)
                    }
                }
            )
        }
        None => {
            html!(
                div class="poem flex flex-col lowercase " {
                    span class=(english_classes) {
                        (english_text)
                    }
                }
            )
        }
    }
}

fn hero_image() -> Markup {
    let class = format!(
        "
            shrink-0 min-w-full min-h-full object-cover
            transform -scale-x-100 lg:scale-x-100 object-right lg:object-left
            {}
        ",
        shared::controllers::parallax::parallax()
    );

    html!(
        (LightDarkImage::new(&ASSETS.hero_image)
            .class(&class)
            .above_the_fold(true)
            .is_largest_contentful_paint(true))
    )
}
