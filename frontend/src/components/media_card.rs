use crate::components::figures::RoboHead;
use common::model::collections::Media;
use common::model::core::{DiscoveryMeta, Movie, TvShow, TvShowDetails, YTChannel};
use yew::{classes, function_component, html, Children, Classes, Html, Properties};

struct CardData {
    pub title: String,
    pub subtitle: String,
    pub description: Option<String>,
    pub fig_path: Option<String>,
    pub baing_reason: Option<String>,
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
            baing_reason: match movie.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason),
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
            baing_reason: match tv_show.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason),
            },
        }
    }
}

impl From<YTChannel> for CardData {
    fn from(channel: YTChannel) -> Self {
        CardData {
            title: channel.name,
            subtitle: "".to_string(),
            description: Some(channel.description),
            fig_path: match channel.details {
                None => None,
                Some(m) => m.backdrop_path,
            },
            baing_reason: match channel.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason),
            },
        }
    }
}

impl From<Media> for CardData {
    fn from(media: Media) -> Self {
        match media {
            Media::Movie(m) => CardData::from(m),
            Media::TvShow(t) => CardData::from(t),
            Media::YTChannel(c) => CardData::from(c),
            _ => unreachable!("Unsupported media type"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MediaCardProps {
    pub media: Media,
    #[prop_or(false)]
    pub lite: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(Children::default())]
    pub children: Children,
}

#[function_component(MediaCard)]
pub fn media_card(props: &MediaCardProps) -> Html {
    let card_data = CardData::from(props.media.to_owned());
    let fig_base = "https://image.tmdb.org/t/p/w500";
    let details = &props.media;
    let class = props.class.clone();
    let mut classes = classes!(
        "text-center",
        "bg-clip-content",
        "border",
        "border-base-content",
        "bg-base-200",
        "card",
        "image-full",
        "grow",
        //"w-3/5",
        class
    );

    if props.lite {
        classes = classes!(classes, "card-lite");
    }

    let card = html! {
        <div class={classes}>
            if let Some(fig_path) = card_data.fig_path {
                <figure><img src={format!("{fig_base}{fig_path}")} /></figure>
            }
            <div class="card-body p-3">
                <h2 class="card-title">
                    <div><p class="truncate">{card_data.title}</p></div>
                    <div class="text-xs">{card_data.subtitle}</div>
                </h2>
                <div>
                    <p class="h-64 overflow-auto">
                        if let Some(desc) = card_data.description {
                            {desc}
                        }
                        if let Some(baing_reason) = card_data.baing_reason {
                            <div class="divider divider-accent">
                                // {"BA!ng Insights"}
                                <figure class="scale-[2.35]">
                                    <RoboHead />
                                </figure>
                            </div>
                            {baing_reason}
                        }
                    </p>
                </div>
                { for props.children.iter() }
            </div>
        </div>
    };

    card
}
