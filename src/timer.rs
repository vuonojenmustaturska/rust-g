use std::cell::RefCell;
use std::time::{Instant, Duration};
use ferris::{AllocWheel, Resolution, Wheel};

thread_local! {
    static TIMERS: RefCell<Timers> = Default::default();
}

struct Timers {
    realwheel: AllocWheel<String>,
    byondwheel: AllocWheel<String>,
    lastrun: Instant
}

impl Default for Timers {
    fn default() -> Self {
        let mut resolutions1 = Vec::<Resolution>::new();
        let mut resolutions2 = Vec::<Resolution>::new();
        resolutions1.push(Resolution::HundredMs);
        resolutions1.push(Resolution::Sec);
        resolutions1.push(Resolution::Min); // = vec!([, Resolution::Sec, Resolution::Min]);
        resolutions2.push(Resolution::HundredMs);
        resolutions2.push(Resolution::Sec);
        resolutions2.push(Resolution::Min); // = vec!([, Resolution::Sec, Resolution::Min]);
        Timers { realwheel: AllocWheel::<String>::new(resolutions1),
        byondwheel: AllocWheel::<String>::new(resolutions2),
        lastrun: Instant::now() }
    }
}

byond_fn! { setup_timers() {
    TIMERS.with(|timers|
        *timers.borrow_mut() = Default::default()
    );
    Some("")
}}

byond_fn! { add_realtime_timer(timerid, delay) {
    match delay.parse::<u64>() {
        Ok(duration) => { TIMERS.with(|timers| timers.borrow_mut().realwheel.start(timerid.to_owned(), Duration::from_millis(100 * duration))); Some("") },
        Err(_e) => Some("parse error")
    }
} }

byond_fn! { add_byondtime_timer(timerid, delay) {
    match delay.parse::<u64>() {
        Ok(duration) => { TIMERS.with(|timers| timers.borrow_mut().byondwheel.start(timerid.to_owned(), Duration::from_millis(100 * duration))); Some("") },
        Err(_e) => Some("parse error")
    }
} }

byond_fn! { del_timer(timerid) {
    TIMERS.with(|timers|
        timers.borrow_mut().byondwheel.stop(timerid.to_owned())
    );
    Some("")
} }

byond_fn! { get_timers() {
    let mut vec = Vec::<String>::new();

    TIMERS.with(|timers|
        {
        let mut timers_mut = timers.borrow_mut();
        vec.append(&mut timers_mut.byondwheel.expire());
        let now = Instant::now();
        let mut diffms = now.duration_since(timers_mut.lastrun).as_millis();
        while diffms >= 100 {
            vec.append(&mut timers_mut.realwheel.expire());
            diffms -= 100;
        }
        timers_mut.lastrun = now;
        }
    );
    Some(vec.join(","))
}  }
