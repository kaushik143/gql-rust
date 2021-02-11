use actix_http::http::{
    header::{CONNECTION, CONTENT_TYPE},
    HeaderMap,
};
use async_graphql::*;

use crate::model::{
    Artwork, ContestCategory, ContestListResponse, ContestQueryInput, ContestSection,
    ContextStruct, Currency, DisplayContest, IMatch, MatchStatus, PreRoundLockContestQueryInput,
    PreRoundLockJoinedContestsResponse, RoundQueryInput, RoundTourResponse, Tour,
};

pub struct Query;
#[Object]
impl Query {
    async fn contest_sections<'a>(
        &self,
        ctx: &'a Context<'_>,
        site: String,
        matchId: i32,
        tourId: i32,
    ) -> FieldResult<Vec<ContestSection>> {
        let roundTourSquad = "/roundTourSquad".to_owned();
        let contestList = "/contest-list".to_owned();
        let contestsLite = "/contest/v1/fetchJoinedContestsLite".to_owned();
        let roundId = matchId.to_owned();
        let tourId = tourId.to_owned();
        let site = &site;
        let roundMessageQuery = RoundQueryInput {
            siteId: 1,
            site: site.to_owned(),
            roundId,
            wlsId: 1,
        };

        let context = ctx.data::<ContextStruct>()?;

        let contestQuery = ContestQueryInput {
            siteId: 1,
            site: site.to_owned(),
            roundId,
            wlsId: 1,
            tourId,
        };

        let preRoundLockQuery = PreRoundLockContestQueryInput {
            siteId: 1,
            wlsId: 1,
            roundId,
            tourId,
            site: site.to_owned(),
            contestDB: "voltdb2".to_owned(),
            roundCalcStatus: 0,
            pcStreamingStatus: 0,
            pcStack: "classic".to_owned(),
            isRoundComplete: 0,
            isRoundLocked: 0,
            isArchive: 0,
        };

        let roundTour: Result<RoundTourResponse, FieldError> =
            post(&context, &roundMessageQuery, roundTourSquad).await;
        let contestListRes: Result<ContestListResponse, FieldError> =
            post(&context, &contestQuery, contestList).await;
        let joinedContest: Result<PreRoundLockJoinedContestsResponse, FieldError> =
            post(&context, &preRoundLockQuery, contestsLite).await;

        match roundTour {
            Ok(res) => match contestListRes {
                Ok(contest) => match joinedContest {
                    Ok(joinContest) => {
                        let contest_section_response =
                            contestSections(&res, &contest, &joinContest);
                        print!("{:?}", "return".to_owned());
                        Ok(contest_section_response)
                    }
                    Err(err) => {
                        println!("{}", err.message());
                        Err(err)
                    }
                },
                Err(err) => {
                    println!("{}", err.message());
                    Err(err)
                }
            },
            Err(err) => {
                println!("{}", err.message());
                Err(err)
            }
        }
    }
}

pub async fn post<T, D>(ctx: &ContextStruct, body: T, url: String) -> Result<D>
where
    T: serde::Serialize,
    D: serde::de::DeserializeOwned,
{
    let base_url = ctx.base_url;
    let final_url = base_url.to_owned() + &url;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Cookie", "ajs_anonymous_id=%2286af43bb-5e00-4cd2-8255-2ccf92bafedd%22; WZRK_G=775da19b01524940b53caff859541c59; _ga=amp-H8eiH9g1yFARZU_jE3mq0w; G_ENABLED_IDPS=google; __csrf=hsvtm; connect.sid=s%3AI5vgVKhdyELwu5H6ZvzYcBuSH_qFw_cT.JUuce2CGUxXHp%2BcVZUWZK6GSx4hkbWJfyNdixGSCRS4; IPL_Offer=variant3; dh_user_id=50155f00-1ab6-11eb-8436-95acf3119b5b".parse().unwrap());
    headers.insert("x-csrf", "hsvtm".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    let res = ctx
        .client
        .post(&final_url)
        .headers(headers)
        .json(&body)
        .send()
        .await;

    match res {
        Err(err) => {
            println!("{:?}", err);
            Err(field_error("Request failure"))
        }
        Ok(data) => {
            let decode = data.json::<D>().await;
            decode
                .map_err(|err| {
                    println!("{:?}", err);
                    return field_error("JSON decode failure");
                })
                .and_then(|gql| Ok(gql))
        }
    }
}

fn contestSections(
    roundTour: &RoundTourResponse,
    contestList: &ContestListResponse,
    _joinedContest: &PreRoundLockJoinedContestsResponse,
) -> Vec<ContestSection> {
    let sections = &contestList.data.sections;
    let contestSectionsResponse = sections
        .iter()
        .map(|section| -> ContestSection {
            ContestSection {
                id: section.sectionConfig.id,
                name: section.sectionConfig.title.to_owned(),
                description: section.sectionConfig.subTitle.to_owned(),
                artwork: vec![Artwork {
                    src: section.sectionConfig.imgURL.to_owned(),
                    height: None,
                    width: None,
                    r#type: None,
                }],
                tag: None,
                totalContestCount: section.sectionConfig.totalCardCount,
                displayContests: section
                    .leagues
                    .iter()
                    .map(|league| -> DisplayContest {
                        DisplayContest {
                            _id: league.id,
                            contestType: league.contestType.to_owned(),
                            contestSize: league.contestSize,
                            currentSize: league.currentSize,
                            entryFee: Currency {
                                amount: league.entryFee,
                                symbol: "\\u20B9".to_owned(),
                            },
                            effectiveEntryFee: None,
                            hasJoined: false,
                            inviteCode: league.inviteCode.to_owned(),
                            isInfiniteEntry: league.contestSize > 10000000,
                            isGuaranteed: league.isGuaranteed == 1,
                            isMultipleEntry: league.multipleEntry == 1,
                            isRecommended: league.isRecommended == 1,
                            numberOfWinners: league.noOfWinners,
                            prizeAmount: Currency {
                                amount: league.prizeAmount,
                                symbol: "\\u20B9".to_owned(),
                            },
                            prizeDisplayText: Some(convertToWordsForIndianCurrency(
                                league.prizeAmount,
                            )),
                            showInvite: false,
                            r#match: IMatch {
                                id: roundTour.round.id,
                                status: {
                                    match roundTour.round.RoundCalcStatus {
                                        2 => MatchStatus::IN_PROGRESS,
                                        3 => MatchStatus::COMPLETED,
                                        4 => MatchStatus::WAITING_FOR_REVIEW,
                                        5 => MatchStatus::ABANDONED,
                                        _ => MatchStatus::NOT_STARTED,
                                    }
                                },
                            },
                            tour: Tour {
                                id: roundTour.tour.TourId,
                                name: roundTour.tour.TourName.to_owned(),
                            },
                            site: "cricket".to_owned(),
                            isPartnerContest: Some(false),
                            contestName: league.contestName.to_owned(),
                            contestCategory: {
                                let free = String::from("free");
                                match league.contestCategory.to_owned() {
                                    free => ContestCategory::FREE,
                                    _ => ContestCategory::PAID,
                                }
                            },
                        }
                    })
                    .collect::<Vec<DisplayContest>>(),
            }
        })
        .collect::<Vec<ContestSection>>();
    return contestSectionsResponse;
}

fn field_error(msg: &str) -> FieldError {
    FieldError::new(msg)
}

fn convertToWordsForIndianCurrency(amount: f64) -> String {
    let divideByCrore = amount as i32 / 10000000;
    let divideByLakh = amount as i32 / 100000;

    if divideByCrore >= 1 {
        return divideByCrore.to_string() + "Crores";
    } else if divideByLakh >= 1 {
        return divideByLakh.to_string() + "Lakhs";
    } else {
        return amount.to_string();
    }
}
