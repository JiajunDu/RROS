use crate::{
    clock::{
        RROS_MONO_CLOCK, RROS_REALTIME_CLOCK,
    },
    thread::{rros_sleep, KthreadRunner},
};

use kernel::{
    c_str,
    prelude::*,
    bindings,
};

fn smp_kfn(i: i32) {
    // rros_sleep(10000);
    const M: usize = 1_000_000_007;
    let (mut first, mut second) = (1, 1);
    for _ in 0..50000000 {
        let temp = (first + second) % M;
        first = second;
        second = temp;
    }
    pr_warn!("[DJJ] the calculation result of thread {:?} is: {:?}", i, second);
}

pub fn test_smp() {

    let mut SMP_KTHREAD_RUNNER_1: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_2: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_3: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_4: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_5: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_6: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_7: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_8: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_9: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_10: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_11: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_12: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_13: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_14: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_15: KthreadRunner = KthreadRunner::new_empty();
    let mut SMP_KTHREAD_RUNNER_16: KthreadRunner = KthreadRunner::new_empty();

    unsafe {
        SMP_KTHREAD_RUNNER_1.init(Box::try_new(||smp_kfn(1)).unwrap());
        SMP_KTHREAD_RUNNER_2.init(Box::try_new(||smp_kfn(2)).unwrap());
        SMP_KTHREAD_RUNNER_3.init(Box::try_new(||smp_kfn(3)).unwrap());
        SMP_KTHREAD_RUNNER_4.init(Box::try_new(||smp_kfn(4)).unwrap());
        SMP_KTHREAD_RUNNER_5.init(Box::try_new(||smp_kfn(5)).unwrap());
        SMP_KTHREAD_RUNNER_6.init(Box::try_new(||smp_kfn(6)).unwrap());
        SMP_KTHREAD_RUNNER_7.init(Box::try_new(||smp_kfn(7)).unwrap());
        SMP_KTHREAD_RUNNER_8.init(Box::try_new(||smp_kfn(8)).unwrap());
        SMP_KTHREAD_RUNNER_1.init(Box::try_new(||smp_kfn(9)).unwrap());
        SMP_KTHREAD_RUNNER_2.init(Box::try_new(||smp_kfn(10)).unwrap());
        SMP_KTHREAD_RUNNER_3.init(Box::try_new(||smp_kfn(11)).unwrap());
        SMP_KTHREAD_RUNNER_4.init(Box::try_new(||smp_kfn(12)).unwrap());
        SMP_KTHREAD_RUNNER_5.init(Box::try_new(||smp_kfn(13)).unwrap());
        SMP_KTHREAD_RUNNER_6.init(Box::try_new(||smp_kfn(14)).unwrap());
        SMP_KTHREAD_RUNNER_7.init(Box::try_new(||smp_kfn(15)).unwrap());
        SMP_KTHREAD_RUNNER_8.init(Box::try_new(||smp_kfn(16)).unwrap());

        SMP_KTHREAD_RUNNER_1.run(c_str!("smp_kthread_1"));
        SMP_KTHREAD_RUNNER_2.run(c_str!("smp_kthread_2"));
        SMP_KTHREAD_RUNNER_3.run(c_str!("smp_kthread_3"));
        SMP_KTHREAD_RUNNER_4.run(c_str!("smp_kthread_4"));
        SMP_KTHREAD_RUNNER_5.run(c_str!("smp_kthread_5"));
        SMP_KTHREAD_RUNNER_6.run(c_str!("smp_kthread_6"));
        SMP_KTHREAD_RUNNER_7.run(c_str!("smp_kthread_7"));
        SMP_KTHREAD_RUNNER_8.run(c_str!("smp_kthread_8"));
        SMP_KTHREAD_RUNNER_1.run(c_str!("smp_kthread_9"));
        SMP_KTHREAD_RUNNER_2.run(c_str!("smp_kthread_10"));
        SMP_KTHREAD_RUNNER_3.run(c_str!("smp_kthread_11"));
        SMP_KTHREAD_RUNNER_4.run(c_str!("smp_kthread_12"));
        SMP_KTHREAD_RUNNER_5.run(c_str!("smp_kthread_13"));
        SMP_KTHREAD_RUNNER_6.run(c_str!("smp_kthread_14"));
        SMP_KTHREAD_RUNNER_7.run(c_str!("smp_kthread_15"));
        SMP_KTHREAD_RUNNER_8.run(c_str!("smp_kthread_16"));
    }
}