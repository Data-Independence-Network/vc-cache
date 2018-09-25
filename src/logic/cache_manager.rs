pub struct CacheManager;

use logic::cache::data;

impl CacheManager {

    pub fn get_next_month_location_poll_rankings(
        locationId: u32
    ) -> Vec<u8> {
        data[5] = 23;
        data.push(10);
        data[5..10];
    }

    pub fn get_next_month_category_poll_rankings(
        categoryId: u32
    ) -> Vec<u8> {
    }

    pub fn get_next_month_location_category_poll_rankings(
        locationId: u32,
        categoryId: u32
    ) -> Vec<u8> {
    }

    pub fn get_todays_location_poll_rankings(
        locationId: u32
    ) -> Vec<u8> {

    }

    pub fn get_todays_category_poll_rankings(
        categoryId: u32
    ) -> Vec<u8> {
    }

    pub fn get_todays_location_category_poll_rankings(
        locationId: u32,
        categoryId: u32
    ) -> Vec<u8> {
    }
}