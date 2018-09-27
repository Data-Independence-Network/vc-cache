use super::super::data::prepend::list::PrependList;
use int_hash::IntHashMap;
use std::collections::HashMap;

pub static mut data: Vec<u32> = Vec::new();

pub static mut lastMonthId: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut LAST_MONTH_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();
pub static mut thisMonthId: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut THIS_MONTH_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();

pub static mut lastWeekId: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut LAST_WEEK_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();
pub static mut thisWeekId: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut THIS_WEEK_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();

pub static mut DAY_BEFORE_YESTERDAY_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();
pub static mut YESTERDAY_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();
pub static mut TODAY_LOCATION_POLL_RANKINGS: Vec<LocationPollRankings> = Vec::new();

pub static mut NEXT_MONTHS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut NEXT_WEEKS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut DAY_AFTER_TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();


pub static mut LAST_MONTH_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();
pub static mut THIS_MONTH_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();

pub static mut LAST_WEEK_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();
pub static mut THIS_WEEK_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();

pub static mut DAY_BEFORE_YESTERDAY_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();
pub static mut YESTERDAY_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();
pub static mut TODAY_CATEGORY_POLL_RANKINGS: Vec<Vec<OneToThreeDPoll>> = Vec::new();

pub static mut NEXT_MONTHS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut NEXT_WEEKS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut TOMORROWS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut DAY_AFTER_TOMORROWS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();


pub static mut LAST_MONTH_CATEGORY_POLLS: LsbShiftTree<[u64]> = LsbShiftTree::new();

pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();
pub static mut TIMEZONE_MODIFICATION_FLAGS: Vec<boolean> = Vec::new();


pub static mut CATEGORY_LAST_MONTH_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_THIS_MONTH_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_LAST_WEEK_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_THIS_WEEK_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_DAY_BEFORE_YESTERDAY_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_YESTERDAY_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_TODAY_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);

pub static mut LOCATION_LAST_MONTH_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_THIS_MONTH_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_LAST_WEEK_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_THIS_WEEK_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_DAY_BEFORE_YESTERDAY_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_YESTERDAY_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_TODAY_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);

/**
 * Random access current poll maps, needed for count and sum increments by the voting servers
 */
pub static mut TODAY_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
pub static mut TODAY_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
pub static mut TODAY_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_WEEK_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_WEEK_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_WEEK_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_MONTH_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_MONTH_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_MONTH_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);

/**
* Polls array by in-cache index, by timezone
*/
pub static mut TODAY_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut TODAY_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut TODAY_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut YESTERDAY_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut YESTERDAY_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut YESTERDAY_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut DAY_B4_YESTERDAY_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut DAY_B4_YESTERDAY_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut DAY_B4_YESTERDAY_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut LAST_WEEK_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut LAST_WEEK_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut LAST_WEEK_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_1_D__POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut LAST_MONTH_1_D__POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut LAST_MONTH_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut LAST_MONTH_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);



pub struct LocationPeriodIds {
    locationIndex: u32,
    categoryIndexMap: IntHashMap<u64, u32>,
}

impl LocationPeriodIds {
    pub fn new(
        locationIndex: u32,
        numCategories: usize,
    ) -> LocationPeriodIds {
        LocationPeriodIds {
            locationIndex,
            categoryIndexMap: IntHashMap::with_capacity(numCategories),
        }
    }
}


/**
Split by timezone:
*/

pub struct LocationPollRankings<'a> {
    location: &'a Vec<OneToThreeDPoll>,
    categoryLocations: &'a Vec<Vec<OneToThreeDPoll>>,
}


pub struct LocationPollPrependLists<'a> {
    location: &'a PrependList<'a>,
    categoryLocations: &'a LsbShiftTree<PrependList<'a>>,
}


pub struct LsbShiftTree<T> {}

impl<T> LsbShiftTree<T> {
    pub fn new() -> LsbShiftTree<T> {
        PollTree {}
    }
}


pub struct CategoryPoll {
    locationId: u32,
    pollId: u32,
}

pub struct CategoryPollRanking {
    pollId: u64,
    voteCount: u32,
}

pub struct LocationCategoryPollRanking {
    locationPeriodPollId: u32,
    voteCount: u32,
}

/*
 * With 64bit Dimension Direction Sums:
 *
 * At least upper 3 bytes in sums will be free, we can use this space for
 * additional threshold counts and flags.  Also the total sum of free
 * bytes will be at least 6 to 18.  This could be used to store additional
 * information about the poll.
 *
 * For example, the positional configuration of a 3D poll can be encoded
 * into a number of configurations.  Lets assume that it would take 2 bytes
 * (64K configurations).  In the
 *
 * With 32 bit sums, they will loose precision after about 300M polls (given
 * that vote could take up to 30 spaces (2*3*5), so may need overflow bytes
 * to keep track of overflow and additional computation is needed:
 *
 * let newVal = oldVal + 24
 * if(newVal < oldVal) {
 *  overflow += 1;
 * }
 *
 * Note for pipe compression having 8u + u32 is actually faster, because
 * only 5 bytes need to be checked and serialized vs 8
 */

pub struct VoteCount {
    /**
    First 5 bits are for timezone, last 3 for for Type
    */
    pollTypeAndTz: u8,
    periodVoteId: u32,
    count: u32,
}

/*
 *
 */
pub struct ThreeDPoll {
    pollId: u64,
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim2dir1Over: u8,
    dim2dir2Over: u8,
    dim3dir1Over: u8,
    dim3dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    dim2dir1Sum: u32,
    dim2dir2Sum: u32,
    dim3dir1Sum: u32,
    dim3dir2Sum: u32,
    voteCount: VoteCount,
}

/*
 *
 */
pub struct TwoDPoll {
    pollId: u64,
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim2dir1Over: u8,
    dim2dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    dim2dir1Sum: u32,
    dim2dir2Sum: u32,
    voteCount: VoteCount,
}

pub struct OneDPoll {
    pollId: u64,
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    voteCount: VoteCount,
}
