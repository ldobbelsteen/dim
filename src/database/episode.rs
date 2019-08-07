use crate::database::media::InsertableMedia;
use crate::database::media::Media;
use crate::database::media::UpdateMedia;
use crate::database::season::Season;
use crate::database::tv::TVShow;
use crate::schema::episode;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct Episode {
    #[serde(skip_serializing)]
    pub id: i32,
    pub seasonid: i32,
    pub episode: i32,

    #[serde(flatten)]
    pub media: Media,
}

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug)]
#[belongs_to(Media, foreign_key = "id")]
#[belongs_to(Season, foreign_key = "seasonid")]
#[table_name = "episode"]
pub struct EpisodeWrapper {
    pub id: i32,
    pub seasonid: i32,
    pub episode: i32,
}

#[derive(Deserialize, Debug)]
pub struct InsertableEpisode {
    #[serde(flatten)]
    pub media: InsertableMedia,
    pub episode: i32,
}

#[derive(Insertable)]
#[table_name = "episode"]
pub struct InsertableEpisodeWrapper {
    pub episode_: i32,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct UpdateEpisode {
    pub seasonid: Option<i32>,
    pub episode: Option<i32>,

    #[serde(flatten)]
    pub media: UpdateMedia,
}

#[derive(AsChangeset)]
#[table_name = "episode"]
pub struct UpdateEpisodeWrapper {
    pub seasonid: Option<i32>,
    pub episode_: Option<i32>,
}

impl Episode {
    pub fn get(
        conn: &diesel::SqliteConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<Json<Episode>, diesel::result::Error> {
        use crate::schema::media;
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let season = Season::belonging_to(&tv_show)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let episode = EpisodeWrapper::belonging_to(&season)
            .filter(episode::dsl::episode_.eq(ep_num))
            .first::<EpisodeWrapper>(conn)?;

        let media = media::dsl::media
            .filter(media::dsl::id.eq(episode.id))
            .first::<Media>(conn)?;

        let result = episode.into(media);

        Ok(Json(result))
    }

    pub fn delete(
        conn: &diesel::SqliteConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::season;
        use crate::schema::tv_show;
        let tv_show = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let season = Season::belonging_to(&tv_show)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let episode = EpisodeWrapper::belonging_to(&season)
            .filter(episode::dsl::episode_.eq(ep_num))
            .first::<EpisodeWrapper>(conn)?;

        Media::delete(conn, episode.id)?;
        Ok(diesel::delete(&episode).execute(conn)?)
    }
}

impl InsertableEpisode {
    pub fn insert(
        &self,
        conn: &diesel::SqliteConnection,
        id: i32,
        season_num: i32,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let season = Season::belonging_to(&tv_show)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let media_id = self.media.new(conn)?;

        let episode: InsertableEpisodeWrapper = self.into();

        let _ = diesel::insert_into(episode::table)
            .values((
                episode::dsl::id.eq(media_id),
                episode,
                episode::dsl::seasonid.eq(season.id),
            ))
            .execute(conn)?;
        Ok(())
    }

    fn into(&self) -> InsertableEpisodeWrapper {
        InsertableEpisodeWrapper {
            episode_: self.episode,
        }
    }
}

impl EpisodeWrapper {
    fn into(self, media: Media) -> Episode {
        Episode {
            id: self.id,
            seasonid: self.seasonid,
            episode: self.episode,
            media: media,
        }
    }
}

impl UpdateEpisode {
    pub fn update(
        &self,
        conn: &diesel::SqliteConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;
        let season = Season::belonging_to(&tv)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let episode = EpisodeWrapper::belonging_to(&season)
            .filter(episode::dsl::episode_.eq(ep_num))
            .first::<EpisodeWrapper>(conn)?;

        let _ = self.media.update(conn, episode.id);
        let _ = diesel::update(&episode).set(self.into()).execute(conn);
        Ok(())
    }

    fn into(&self) -> UpdateEpisodeWrapper {
        UpdateEpisodeWrapper {
            seasonid: self.seasonid,
            episode_: self.episode,
        }
    }
}
