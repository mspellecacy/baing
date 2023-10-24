use common::model::collections::Media;
use common::model::core::{Movie, TvShow, TvShowDetails};
use yew::{function_component, html, Children, Html, Properties};

struct CardData {
    pub title: String,
    pub subtitle: String,
    pub description: Option<String>,
    pub fig_path: Option<String>,
}

impl From<Movie> for CardData {
    fn from(movie: Movie) -> Self {
        CardData {
            title: movie.name,
            subtitle: format!("({})", movie.year),
            description: movie.details.as_ref().map(|m| m.overview.to_owned()),
            fig_path: match movie.details {
                None => None,
                Some(m) => m.backdrop_path,
            },
        }
    }
}

impl From<TvShow> for CardData {
    fn from(tv_show: TvShow) -> Self {
        CardData {
            title: tv_show.name,
            subtitle: format!("({})", tv_show.first_air_date),
            description: tv_show.details.as_ref().map(|t| t.overview.to_owned()),
            fig_path: match tv_show.details {
                None => None,
                Some(t) => t.backdrop_path,
            },
        }
    }
}

impl From<Media> for CardData {
    fn from(media: Media) -> Self {
        match media {
            Media::Movie(m) => CardData::from(m),
            Media::TvShow(t) => CardData::from(t),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MediaCardProps {
    pub media: Media,
    #[prop_or(Children::default())]
    pub children: Children,
}

#[function_component(MediaCard)]
pub fn media_card(props: &MediaCardProps) -> Html {
    let card_data = CardData::from(props.media.to_owned());
    let fig_base = "https://image.tmdb.org/t/p/w500";
    let details = &props.media;

    let card = html! {
        <div class="text-center border border-base-content bg-base-200 card image-full">
            if let Some(fig_path) = card_data.fig_path {
                <figure><img style="transform: scale(1.25);" class="p-0" src={format!("{fig_base}{fig_path}")} /></figure>
            }
            <div class="card-body">
                <h2 class="card-title">
                    <div>{card_data.title}</div>
                    <div class="text-xs">{card_data.subtitle}</div>
                </h2>
                if let Some(desc) = card_data.description {
                    <p class="h-64 overflow-y-auto">{desc}</p>
                } else {
                    <p class="h-64">{"Description Unavailable"}</p>
                }
                { for props.children.iter() }
            </div>
        </div>
    };

    card
}
