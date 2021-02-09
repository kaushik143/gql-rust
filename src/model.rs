use juniper::{GraphQLObject, GraphQLEnum};
#[derive(serde::Deserialize, Debug)]
pub struct Matches {
    id: i32,
    RoundId: i32,
    MatchId: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct Round {
    pub id: i32,
    pub RoundCalcStatus: i32
}
#[derive(serde::Deserialize, Debug)]
pub struct RoundTour {
    pub TourId: i32,
    pub TourName: String,
    TourShortName: String,
    TourTag: String,
    TourFormat: String
}
#[derive(serde::Deserialize, Debug)]
pub struct WLS {
    id: i32,
    WLSUrl: String
}

#[derive(serde::Deserialize, Debug)]
pub struct RoundResponse {
    round: Round,
    tour: RoundTour,
    wls: Vec<WLS>,
    matches: Vec<Matches>,
    summary: Summary
}

#[derive(serde::Deserialize, Debug)]
pub struct Summary {
    pcStack: String
}
#[derive(serde::Deserialize, Debug)]
pub struct Squad {
    GameId: i32,
    GameName: String
}
#[derive(serde::Deserialize, Debug)]
pub struct TossResult {
    tossWonBy: Option<String>,
    tossDecision: Option<String>
}
#[derive(serde::Deserialize, Debug)]
pub struct RoundTourResponse {
    pub round: Round,
    pub tour: RoundTour,
    squads: Vec<Squad>,
    tossResult: TossResult
}

#[derive(serde::Deserialize, Debug)]
pub struct ContestListData {
    pub seeAllConfig: Vec<SeeAllConfig>,
    pub sections: Vec<Section>
}

#[derive(serde::Deserialize, Debug)]
pub struct League {
    pub id: i32,
    pub contestId: i32,
    pub contestType: String,
    pub contestName: Option<String>,
    pub contestCategory: String,
    pub currentSize: i32,
    pub contestSize: i32,
    pub entryFee: f64,
    pub inviteCode: String,
    pub isGuaranteed: i32,
    pub multipleEntry: i32,
    pub isRecommended: i32,
    pub noOfWinners: i32,
    pub prizeAmount: f64,
}

#[derive(serde::Deserialize, Debug)]
pub struct ContestListResponse{
    pub data: ContestListData,
}

#[derive(serde::Deserialize, Debug)]
pub struct SectionConfig{
    pub id: i32,
    pub title: String,
    pub subTitle: String,
    pub imgURL: String,
    pub totalCardCount: i32,
    pub showCardCount: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct Section{
    pub sectionConfig: SectionConfig,
    pub leagues: Vec<League>
}

#[derive(serde::Deserialize, Debug)]
pub struct SeeAllConfig{
    id: i32
}
#[derive(serde::Deserialize, Debug)]
pub struct Msg{
    MsgCode: String,
    MsgShowUp: Option<String>,
    MsgType: Option<String>,
    MsgTitle: Option<String>,
    MsgText: Option<String>,
}
#[derive(serde::Deserialize, Debug)]
pub struct PreRoundLockJoinedContestsResponse{
    success: i32,
    errCode: String,
    msg: Msg,
    joinedContests: Vec<League>,
    error: Option<String>,
}
///

/// Contest sections response
#[derive(serde::Deserialize, Debug)]
pub struct JoinedLeagues{
    Leagues: Vec<League>,
    MatchStatus: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RoundInfo{
    TourName: String,
    TourTag: String,
    TourFormat: String,
    MatchStatus: String,
    RoundStartTime: String,
    currentTime: String,
    roundStartTime: String,
    showPlayerImages: i32,
}

#[derive(GraphQLObject, serde::Deserialize, Debug)]
pub struct Tag{
    text: Option<String>
}

#[derive(GraphQLEnum, serde::Deserialize, Debug)]
pub enum ContestCategory {
    PAID,
    FREE
}
#[derive(GraphQLObject, serde::Deserialize, Debug)]
  pub struct Currency{
    pub amount: f64,
    pub symbol: String,
  }

  #[derive(GraphQLEnum, serde::Deserialize, Debug)]
  pub enum MatchStatus {
    ABANDONED,
    COMPLETED,
    IN_PROGRESS,
    NOT_STARTED,
    WAITING_FOR_REVIEW,
    UP_COMING
  }
  #[derive(GraphQLObject, serde::Deserialize, Debug)]
  pub struct IMatch{
    pub(crate) id: i32,
    pub(crate) status: MatchStatus,
  }

  #[derive(GraphQLObject, serde::Deserialize, Debug)]
pub struct Tour{
    pub(crate) id: i32,
    pub(crate) name: String,
  }

  #[derive(GraphQLObject, serde::Deserialize, Debug)]
  pub struct DisplayContest{
    pub contestType: String,
    pub isPartnerContest: Option<bool>,
    pub contestName: Option<String>,
    pub contestCategory: ContestCategory,
    pub contestSize: i32,
    pub currentSize: i32,
    pub entryFee: Currency,
    pub effectiveEntryFee: Option<Currency>,
    pub hasJoined: bool,
    pub _id: i32,
    pub inviteCode: String,
    pub isInfiniteEntry: bool,
    pub isGuaranteed: bool,
    pub isMultipleEntry: bool,
    pub isRecommended: bool,
    pub numberOfWinners: i32,
    pub prizeAmount: Currency,
    pub prizeDisplayText: Option<String>,
    pub showInvite: bool,
    pub r#match: IMatch,
    pub tour: Tour,
    pub site: String,
  }
  #[derive(GraphQLObject, serde::Deserialize, Debug)]
  pub struct Artwork{
    pub src: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub r#type: Option<String>,
  }
  #[derive(GraphQLObject)]
  pub struct ContestSection{
    pub id: i32,
    pub name: String,
    pub description: String,
    pub artwork: Vec<Artwork>,
    pub tag: Option<Tag>,
    pub totalContestCount: i32,
    pub displayContests: Vec<DisplayContest>,
  }

