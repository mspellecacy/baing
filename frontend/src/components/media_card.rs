use serde_json::map::Entry::Vacant;
use crate::components::figures::{PlayButton, RoboHead};
use common::model::collections::{IsMedia, Media};
use common::model::core::{DiscoveryMeta, Movie, OnlineContent, TvShow, TvShowDetails, YTChannel};
use yew::{classes, function_component, html, Children, Classes, Html, Properties};

struct CardData {
    pub title: String,
    pub subtitle: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub fig_path: Option<String>,
    pub baing_reason: Option<String>,
    // pub baing_streamers: Option<String>,
}

impl From<Movie> for CardData {
    fn from(movie: Movie) -> Self {
        CardData {
            title: movie.name,
            subtitle: format!("({})", movie.year),
            description: movie.details.as_ref().map(|m| m.overview.to_owned()),
            link: None,
            fig_path: match movie.details {
                None => None,
                Some(m) => m.backdrop_path,
            },
            baing_reason: match &movie.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason.clone()),
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
            link: None,
            fig_path: match tv_show.details {
                None => None,
                Some(t) => t.backdrop_path,
            },
            baing_reason: match &tv_show.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason.clone()),
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
            link: None,
            fig_path: match channel.details {
                None => None,
                Some(m) => m.backdrop_path,
            },
            baing_reason: match &channel.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason.clone()),
            },
        }
    }
}

impl From<OnlineContent> for CardData {
    fn from(oc: OnlineContent) -> Self {
        CardData {
            title: oc.name,
            subtitle: String::from("toots"),
            description: Some(oc.description),
            link: Some(oc.url),
            fig_path: None,
            baing_reason: match &oc.baing_meta {
                None => None,
                Some(discovery_meta) => Some(discovery_meta.reason.clone()),
            },
        }
    }
}

impl TryFrom<Media> for CardData {
    type Error = &'static str;

    fn try_from(media: Media) -> Result<Self, Self::Error> {
        match media {
            Media::Movie(m) => Ok(CardData::from(m)),
            Media::TvShow(t) => Ok(CardData::from(t)),
            Media::YTChannel(c) => Ok(CardData::from(c)),
            Media::OnlineContent(oc) => Ok(CardData::from(oc)),
            _ => Err("Unsupported media type"),
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
    let card_data = CardData::try_from(props.media.to_owned()).expect("Missing Card Data");
    let fig_base = "https://image.tmdb.org/t/p/w500";
    let details = &props.media;
    let class = props.class.clone();
    let mut classes = classes!(
        "card",
        "text-center",
        "bg-clip-content",
        "border",
        "border-base-content",
        "bg-base-200",
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
                     if let Some(link) = card_data.link {
                        <div>
                            <a class="link" href={link} target="_blank">
                                <p class="truncate">{card_data.title}</p>
                            </a>
                        </div>
                    } else {
                       <div><p class="truncate">{card_data.title}</p></div>
                    }
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
