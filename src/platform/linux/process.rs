use libc::{SCHED_OTHER, sched_getscheduler, sched_param, sched_setscheduler};

pub fn prepare_process() {
    unsafe {
        println!("Before: {}", sched_getscheduler(0));

        let param = sched_param { sched_priority: 0 };

        let ret = sched_setscheduler(0, SCHED_OTHER, &param);

        println!("Return: {}", ret);

        if ret != 0 {
            println!("Error: {}", std::io::Error::last_os_error());
        }

        println!("After: {}", sched_getscheduler(0));
    }
}
