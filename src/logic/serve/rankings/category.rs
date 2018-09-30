use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::server::codes;
use super::super::super::super::data::byte_counts::ByteCounts;

use std::mem::transmute;

const PAGE_SIZE: usize = 1024;

const MAX_NUM_POLL_BYTES: usize = 4;

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
            response.push(MAX_NUM_POLL_BYTES as u8);
            let mut pollRankings: Vec<VoteCount> = cache::TODAY_CATEGORY_POLL_RANKINGS.get(categoryIndex);

            match maxNumPollBytes
            get4ByteRecentPolls(pollRankings, PAGE_SIZE * blockIndex, )

            return response;
        }
    }

}

fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>
) {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut byteCounts = ByteCounts::new(PAGE_SIZE);

    for x in  0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);
                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // In this case tzAndPeriodPollId is the total number of polls in
                    // the period across all time zones
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes);
                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    byteCounts.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    byteCounts.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    byteCounts.add2();
                } else {
                    response.push(countBytes[3]);
                    byteCounts.add1();
                }
            }
        }
    }


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
