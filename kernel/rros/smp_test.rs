use crate::{
    clock::{
        RROS_MONO_CLOCK, RROS_REALTIME_CLOCK,
    },
    thread::KthreadRunner,
};

use kernel::{
    c_str,
    prelude::*,
    bindings,
};

static mut SMP_KTHREAD_RUNNER_1: KthreadRunner = KthreadRunner::new_empty();
static mut SMP_KTHREAD_RUNNER_2: KthreadRunner = KthreadRunner::new_empty();
static mut SMP_KTHREAD_RUNNER_3: KthreadRunner = KthreadRunner::new_empty();
static mut SMP_KTHREAD_RUNNER_4: KthreadRunner = KthreadRunner::new_empty();


fn smp_kfn_1() {
    let (mut first, mut second) = (1, 1);
    for _ in 0..1000000000 {
        let temp = first + second;
        first = second;
        second = temp;
        // unsafe { bindings::usleep_range(100, 105); }
        first = 1;
        second = 2;
    }
    pr_warn!("[DJJSMP] 1 second is: {:?}", second);
}

fn smp_kfn_2() {
    let (mut first, mut second) = (1, 1);
    for _ in 0..1000000000 {
        let temp = first + second;
        first = second;
        second = temp;
        // unsafe { bindings::usleep_range(200, 205); }
        first = 1;
        second = 2;
    }
    pr_warn!("[DJJSMP] 2 second is: {:?}", second);
}

fn smp_kfn_3() {
    let (mut first, mut second) = (1, 1);
    for _ in 0..1000000000 {
        let temp = first + second;
        first = second;
        second = temp;
        // unsafe { bindings::usleep_range(200, 205); }
        first = 1;
        second = 2;
    }
    pr_warn!("[DJJSMP] 3 second is: {:?}", second);
}

fn smp_kfn_4() {
    let (mut first, mut second) = (1, 1);
    for _ in 0..1000000000 {
        let temp = first + second;
        first = second;
        second = temp;
        // unsafe { bindings::usleep_range(200, 205); }
        first = 1;
        second = 2;
    }
    pr_warn!("[DJJSMP] 4 second is: {:?}", second);
}

pub fn test_smp() {
    let mono_read_result1 = unsafe { RROS_MONO_CLOCK.read() };
    let rt_read_result1 = unsafe { RROS_REALTIME_CLOCK.read() };
    pr_warn!("[DJJ] 1 mono: {:?}, realtime: {:?}", mono_read_result1, rt_read_result1);

    unsafe {
        SMP_KTHREAD_RUNNER_1.init(Box::try_new(smp_kfn_1).unwrap());
        SMP_KTHREAD_RUNNER_2.init(Box::try_new(smp_kfn_2).unwrap());
        SMP_KTHREAD_RUNNER_3.init(Box::try_new(smp_kfn_3).unwrap());
        SMP_KTHREAD_RUNNER_4.init(Box::try_new(smp_kfn_4).unwrap());

        SMP_KTHREAD_RUNNER_1.run(c_str!("smp_kthread_1"));
        SMP_KTHREAD_RUNNER_2.run(c_str!("smp_kthread_2"));
        SMP_KTHREAD_RUNNER_3.run(c_str!("smp_kthread_3"));
        SMP_KTHREAD_RUNNER_4.run(c_str!("smp_kthread_4"));
    }

    let mono_read_result2 = unsafe { RROS_MONO_CLOCK.read() };
    let rt_read_result2 = unsafe { RROS_REALTIME_CLOCK.read() };
    pr_warn!("[DJJ] 2 mono: {:?}, realtime: {:?}", mono_read_result2, rt_read_result2);

}
