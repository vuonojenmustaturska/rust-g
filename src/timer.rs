use std::time::Duration;
use std::thread;

use pendulum::{Pendulum, HashedWheelBuilder, HashedWheel};

use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static REALTIMERS: RefCell<HashedWheel<String>> = RefCell::new(HashedWheelBuilder::default().with_tick_duration(Duration::from_millis(100)).build::<String>());
    static BYONDTIMERS: RefCell<HashedWheel<String>> = RefCell::new(HashedWheelBuilder::default().with_tick_duration(Duration::from_millis(100)).build::<String>());
    static TIMERENTRIES: RefCell<HashMap::<String, pendulum::Token>> = RefCell::new(HashMap::<String, pendulum::Token>::new());
}

struct Timers {
    realwheel: HashedWheel<String>,
    byondwheel: HashedWheel<String>,
    entries: HashMap<String, pendulum::Token>
}

byond_fn! { add_realtime_timer(timerid, delay) {
    REALTIMERS.with(|timers|
        TIMERENTRIES.with(|entries|
            entries.borrow_mut().insert(timerid.to_string(), timers.borrow_mut().insert_timeout(Duration::from_millis(100 * delay.parse::<u64>().unwrap()), timerid.to_string()).unwrap()))
    );
    Some("")
} }

byond_fn! { add_byondtime_timer(timerid, delay) {
    BYONDTIMERS.with(|timers|
        TIMERENTRIES.with(|entries|
            entries.borrow_mut().insert(timerid.to_string(), timers.borrow_mut().insert_timeout(Duration::from_millis(100 * delay.parse::<u64>().unwrap()), timerid.to_string()).unwrap()))
    );
    Some("")
} }

byond_fn! { del_timer(timerid) {
    REALTIMERS.with(|timers|
        TIMERENTRIES.with(|entries|
        timers.borrow_mut().remove_timeout(entries.borrow_mut().remove(timerid).unwrap())
    ))
} }

byond_fn! { get_timers(byondtick) {
    let mut vec = Vec::<String>::new();
    let byondtick = byondtick.parse::<u64>().unwrap();
    REALTIMERS.with(|timers|
    TIMERENTRIES.with(|entries|
        while ()
        while let Some(expired) = timers.borrow_mut().expired_timeout() {
            vec.push(expired.clone());
            entries.borrow_mut().remove(&expired);
        }
    )
    );
    Some(vec.join(","))
}  }
