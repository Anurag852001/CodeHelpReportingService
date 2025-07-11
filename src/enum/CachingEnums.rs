use std::time::Duration;
use once_cell::sync::Lazy;

use moka::future::Cache;

pub enum CachingEnums {
    TwoHundredDays,
    TwoDays,
    FiveMins,
    TwoHours,
    OneMinute
}
static CACHE_FOR_TWO_HUNDRED_DAYS: Lazy<Cache<String,String>> = Lazy::new (|| {
    Cache::builder().time_to_live(Duration::from_secs(CachingEnums::TwoHundredDays.get_seconds())).build()
});

static CACHE_FOR_TWO_DAYS: Lazy<Cache<String,String>> = Lazy::new (|| {
    Cache::builder().time_to_live(Duration::from_secs(CachingEnums::TwoDays.get_seconds())).build()
});

static CACHE_FOR_FIVE_MINS:Lazy<Cache<String,String>> = Lazy::new (||{
    Cache::builder().time_to_live(Duration::from_secs(CachingEnums::FiveMins.get_seconds())).build()
});
static CACHE_FOR_TWO_HOURS:Lazy<Cache<String,String>> = Lazy::new (||{
    Cache::builder().time_to_live(Duration::from_secs(CachingEnums::TwoHours.get_seconds())).build()
});

static CACHE_FOR_ONE_MINUTE:Lazy<Cache<String,String>> = Lazy::new (||{
    Cache::builder().time_to_live(Duration::from_secs(CachingEnums::OneMinute.get_seconds())).build()
});

pub fn get_cache(ce: CachingEnums) -> &'static Cache<String, String> {
    match ce {
        CachingEnums::TwoHundredDays => {
            &CACHE_FOR_TWO_HUNDRED_DAYS
        }
        CachingEnums::TwoDays => {
            &CACHE_FOR_TWO_DAYS
        }
        CachingEnums::FiveMins => {
            &CACHE_FOR_FIVE_MINS
        }
        CachingEnums::TwoHours => {
            &CACHE_FOR_TWO_HOURS
        }
        CachingEnums::OneMinute => {
            &CACHE_FOR_ONE_MINUTE
        }
    }
}
impl CachingEnums {
    pub fn get_seconds(&self) -> u64 {
        match self {
            CachingEnums::TwoHundredDays =>  17_280_000,
            CachingEnums::TwoDays =>  172_800,
            CachingEnums::FiveMins => 300,
            CachingEnums::TwoHours => 7200,
            CachingEnums::OneMinute => 60
        }
    }
}