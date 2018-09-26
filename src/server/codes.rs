pub static RESPONSE_INVALID_REQUEST:u8 = 129;
pub static RESPONSE_DATA: u8 = 1;



pub static URL_NEXT_MONTHS_LOCATION_POLLS: &str = "0";
pub static URL_NEXT_WEEKS_LOCATION_POLLS: &str = "1";
pub static URL_TOMORROWS_LOCATION_POLLS: &str = "2";
pub static URL_DAY_AFTER_TOMORROWS_LOCATION_POLLS: &str = "3";


pub static URL_THIS_MONTHS_LOCATION_POLL_RANKINGS: &str = "4";
pub static URL_THIS_WEEKS_LOCATION_POLL_RANKINGS: &str = "5";
pub static URL_TODAYS_LOCATION_POLL_RANKINGS: &str = "6";


pub static URL_LAST_MONTHS_LOCATION_POLL_RANKINGS: &str = "7";
pub static URL_LAST_WEEKS__LOCATION_POLL_RANKINGS: &str = "8";
pub static URL_YESTERDAYS_LOCATION_POLL_RANKINGS: &str = "9";
pub static URL_DAY_BEFORE_YESTERDAY_LOCATION_POLL_RANKINGS: &str = "_";




pub static URL_NEXT_MONTHS_CATEGORY_POLLS: &str = "a";
pub static URL_NEXT_WEEKS_CATEGORY_POLLS: &str = "b";
pub static URL_TOMORROWS_CATEGORY_POLLS: &str = "c";
pub static URL_DAY_AFTER_TOMORROWS_CATEGORY_POLLS: &str = "d";


pub static URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS: &str = "e";
pub static URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS: &str = "f";
pub static URL_TODAYS_CATEGORY_POLL_RANKINGS: &str = "g";


pub static URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS: &str = "h";
pub static URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS: &str = "i";
pub static URL_YESTERDAYS_CATEGORY_POLL_RANKINGS: &str = "j";
pub static URL_DAY_BEFORE_YESTERDAY_CATEGORY_POLL_RANKINGS: &str = "k";




pub static URL_NEXT_MONTHS_LOCATION_CATEGORY_POLLS: &str = "A";
pub static URL_NEXT_WEEKS_LOCATION_CATEGORY_POLLS: &str = "B";
pub static URL_TOMORROWS_LOCATION_CATEGORY_POLLS: &str = "C";
pub static URL_DAY_AFTER_TOMORROWS_LOCATION_CATEGORY_POLLS: &str = "D";


pub static URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS: &str = "E";
pub static URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS: &str = "F";
pub static URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS: &str = "G";


pub static URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS: &str = "H";
pub static URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS: &str = "I";
pub static URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS: &str = "J";
pub static URL_DAY_BEFORE_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS: &str = "K";


/**  Time zones **/
//UTC Offset	Locations	Example Name	Example Location
pub static UTC_PLUS_14: usize = 0; //	Christmas Island/Kiribati	LINT	Kiritimati
pub static UTC_PLUS_13: usize = 1;	// Tonga and 3 more	TOT	Nukualofa
pub static UTC_PLUS_12_45: usize = 2;	// Chatham Islands/New Zealand	CHAST	Chatham Islands
pub static UTC_PLUS_12: usize = 3;	// New Zealand with exceptions and 9 more	ANAT	Anadyr
pub static UTC_PLUS_11: usize = 4;	// small region of Russia and 6 more	SBT	Honiara
pub static UTC_PLUS_10_30: usize = 5;	// Lord Howe Island/Australia	LHST	Lord Howe Island
pub static UTC_PLUS_10: usize = 6;	// much of Australia and 6 more	AEST	Melbourne
pub static UTC_PLUS_9_30: usize = 7; // some regions of Australia	ACST	Adelaide
pub static UTC_PLUS_9: usize = 8; // Japan, South Korea and 5 more	JST	Tokyo
pub static UTC_PLUS_8_45: usize = 9; // Western Australia/Australia	ACWST	Eucla
pub static UTC_PLUS_8: usize = 10; // China, Philippines and 11 more	CST	Beijing
pub static UTC_PLUS_7: usize = 11; // much of Indonesia, Thailand and 7 more	WIB	Jakarta
pub static UTC_PLUS_6_30: usize = 12; // Myanmar and Cocos Islands	MMT	Yangon
pub static UTC_PLUS_6: usize = 13; // Bangladesh and 6 more	BST	Dhaka
pub static UTC_PLUS_5_45: usize = 14; // Nepal	NPT	Kathmandu
pub static UTC_PLUS_5_30: usize = 15; // India and Sri Lanka	IST	New Delhi
pub static UTC_PLUS_5: usize = 16;	// Pakistan and 8 more	UZT	Tashkent
pub static UTC_PLUS_4_30: usize = 17; // Afghanistan	AFT	Kabul
pub static UTC_PLUS_4: usize = 18; // Azerbaijan and 8 more	GST	Dubai
pub static UTC_PLUS_3_30: usize = 19; // Iran	IRST	Tehran
pub static UTC_PLUS_3: usize = 20; // Greece and 37 more	MSK	Moscow
pub static UTC_PLUS_2: usize = 21; // Germany and 47 more	CEST	Brussels
pub static UTC_PLUS_1: usize = 22; // United Kingdom and 23 more	BST	London
pub static UTC_PLUS_0: usize = 23; // Iceland and 16 more	GMT	Accra
pub static UTC_MINUS_1: usize = 24; // Cabo Verde	CVT	Praia
pub static UTC_MINUS_2: usize = 25; // most of Greenland and 3 more	WGST	Nuuk
pub static UTC_MINUS_2_30: usize = 26; // Newfoundland and Labrador/Canada	NDT	St. John's
pub static UTC_MINUS_3: usize = 27; // most of Brazil, Argentina and 9 more	ART	Buenos Aires
pub static UTC_MINUS_4: usize = 28; // regions of USA and 32 more	EDT	New York
pub static UTC_MINUS_5: usize = 29; // regions of USA and 10 more	CDT	Mexico City
pub static UTC_MINUS_6: usize = 30; // small region of USA and 9 more	CST	Guatemala City
pub static UTC_MINUS_7: usize = 31; // regions of USA and 2 more	PDT	Los Angeles
pub static UTC_MINUS_8: usize = 32; // Alaska/USA and 2 more	AKDT	Anchorage
pub static UTC_MINUS_9: usize = 33; // Alaska/USA and regions of French Polynesia	HDT	Adak
pub static UTC_MINUS_9_30: usize = 34; // Marquesas Islands/French Polynesia	MART	Taiohae
pub static UTC_MINUS_10: usize = 35; // Hawaii/USA and 2 more	HST	Honolulu
pub static UTC_MINUS_11: usize = 36; // American Samoa and 2 more	NUT	Alofi
pub static UTC_MINUS_12: usize = 37; // much of US Minor Outlying Islands	AoE	Baker Island

