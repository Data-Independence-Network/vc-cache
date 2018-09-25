use super::super::data::prepend::list::PrependList;

pub static mut data: Vec<u32> = Vec::new();

pub static mut LAST_MONTH_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut THIS_MONTH_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();

pub static mut LAST_WEEK_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut THIS_WEEK_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();

pub static mut DAY_BEFORE_YESTERDAY_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut YESTERDAY_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();
pub static mut TODAY_LOCATION_POLL_RANKINGS: Vec<u32> = Vec::new();

pub static mut NEXT_MONTHS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut NEXT_WEEKS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut DAY_AFTER_TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();



pub static mut LAST_MONTH_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut THIS_MONTH_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();

pub static mut LAST_WEEK_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut THIS_WEEK_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();

pub static mut DAY_BEFORE_YESTERDAY_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut YESTERDAY_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();
pub static mut TODAY_CATEGORY_POLL_RANKINGS: LsbShiftTree<[CategoryPollRanking]> = LsbShiftTree::new();

pub static mut NEXT_MONTHS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut NEXT_WEEKS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut TOMORROWS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();
pub static mut DAY_AFTER_TOMORROWS_CATEGORY_POLLS: LsbShiftTree<PrependList> = LsbShiftTree::new();



pub static mut LAST_MONTH_CATEGORY_POLLS: LsbShiftTree<[u64]> = LsbShiftTree::new();

pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();


pub struct TimezoneSlicedPolls {
    location: [LocationCategoryPollRanking],
    categoryLocations: LsbShiftTree<[u64]>,
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


