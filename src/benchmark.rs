use rand::Rng;


pub fn benchmark() {
    let n = 100000;
    let mut l1: Vec<u8> = rand::thread_rng()
                            .sample_iter(rand::distributions::Uniform::new(0, 17))
                            .take(n)
                            .collect();
    let mut l2: Vec<u32> = rand::thread_rng()
                            .sample_iter(rand::distributions::Uniform::new(0, 17))
                            .take(n)
                            .collect();
    let mut l3: Vec<u32> = rand::thread_rng()
                            .sample_iter(rand::distributions::Uniform::new(0, 17))
                            .take(n)
                            .map(|val| 2u32.pow(val))
                            .collect();
    let mut l4: Vec<u32> = rand::thread_rng()
                            .sample_iter(rand::distributions::Uniform::new(0, 17))
                            .take(n)
                            .map(|val| 2u32.pow(val))
                            .collect();
    let mut l5: Vec<u32> = rand::thread_rng()
                            .sample_iter(rand::distributions::Uniform::new(0, 17))
                            .take(n)
                            .map(|val| 2u32.pow(val))
                            .collect();

    // println!("{:?}", l1);
    // println!("{:?}", l2);
    // println!("{:?}", l3);

    let start_time = std::time::Instant::now();

    for i in 0..n {
        l1[i] = l1[i] + 1;
    }

    let time_spend = start_time.elapsed().as_nanos();

    println!("Time spend: {} ns", time_spend);

    let start_time = std::time::Instant::now();
    for i in 0..n {
        l2[i] = l2[i] + 1;
    }
    let time_spend = start_time.elapsed().as_nanos();

    println!("Time spend: {} ns", time_spend);

    let start_time = std::time::Instant::now();
    for i in 0..n {
        l3[i] = l3[i] * 2;
    }
    let time_spend = start_time.elapsed().as_nanos();

    println!("Time spend: {} ns", time_spend);

    let start_time = std::time::Instant::now();
    for i in 0..n {
        l4[i] = l4[i] << 1;
    }
    let time_spend = start_time.elapsed().as_nanos();

    println!("Time spend: {} ns", time_spend);


    let start_time = std::time::Instant::now();
    let l5: Vec<u32> = l5.iter().map(|val| val << 1).collect();
    let time_spend = start_time.elapsed().as_nanos();

    println!("Time spend: {} ns", time_spend);
}
