use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::server::codes;

use std::mem::transmute;

static pageSize: usize = 1024;

static maxNumPollBytes: usize = 4;

pub fn get_todays_category_rankings_by_global_id(
    vcDayId: u32,
    globalCategoryId: &u64,
    blockIndex: u32,
) -> Vec<u8> {
    const result: Option<&usize> = cache::CATEGORY_TODAY_INDEX_MAP.get(globalCategoryId);
    match result {
        None => {
            return codes::INVALID_CATEGORY_RESPONSE;
        },
        Some(categoryIndex) => {
            let mut response: Vec<u8> = Vec::with_capacity(2048);
            response.push(maxNumPollBytes as u8);
            let mut pollRankings: Vec<VoteCount> = cache::TODAY_CATEGORY_POLL_RANKINGS.get(categoryIndex);



            for x in pollRankings {

            }

            unsafe {
                transmute
            }

            return codes::INVALID_DATA_FORMAT_RESPONSE;
        },
    }

}

fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    pageSize: usize,
    mut response: Vec<u8>
) {
    let mut iterator = pollRankings.iter().skip(startingIndex);

    loop {
        match iterator.next() {
            None => return,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let raw_bytes: [i8; 4] = unsafe { std::mem::transmute(voteCount.pollTypeAndTz); };
            }
        }
    }

    for byte in &raw_bytes {
        println!("{}", byte);
    }
    return [b1, b2, b3, b4]
}

pub fn get_todays_category_rankings_by_cache_index(
    vcDayId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_yesterdays_category_rankings_by_global_id(
    vcDayId: u32,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_yesterdays_category_rankings_by_cache_index(
    vcDayId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_day_b4_yesterdays_category_rankings_by_global_id(
    vcDayId: u32,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_day_b4_yesterdays_category_rankings_by_cache_index(
    vcDayId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_weeks_category_rankings_by_global_id(
    vcWeekId: u32,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_weeks_category_rankings_by_cache_index(
    vcWeekId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_weeks_category_rankings_by_global_id(
    vcWeekId: u32,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_weeks_category_rankings_by_cache_index(
    vcWeekId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_months_category_rankings_by_global_id(
    vcMonthId: u32,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_this_months_category_rankings_by_cache_index(
    vcMonthId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_months_category_rankings_by_global_id(
    vcMonthId: u32,
    globalCategoryId: u64,
    blockIndex: u32,
) -> Vec<u8> {

}

pub fn get_last_months_category_rankings_by_cache_index(
    vcMonthId: u32,
    categoryCacheIndex: u32,
    blockIndex: u32,
) -> Vec<u8> {

}
