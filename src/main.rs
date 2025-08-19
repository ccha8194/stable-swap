use stable_swaplib::*;

fn main() {
    
    let mut pool = StableSwapPool::new(vec![300, 1000], 85, 6).unwrap();

    let dx = 400;

    let d = pool.get_d().unwrap();
    

    let dy = pool.get_dy(0, 1, dx, pool.fee_bps).unwrap();
    let slippage = pool.calculate_slippage_bps(0, 1, dx);
    

    println!("Test 1: 300 Token A, 1000 Token B, A = 85, fee_bps = 6, dx = 400");
    println!("dy: {}", dy);
    println!("slippage: {}", slippage);

    println!("\n");
    println!("Comparison with constant product formula:");

    let cp_pool = ConstantProductPool::new(vec![300, 1000], 6).unwrap();
    let cp_dy = cp_pool.get_dy(0, 1, dx).unwrap();
    let cp_slippage = cp_pool.calculate_slippage_bps(0, 1, dx);

    println!("dy: {} vs {}", cp_dy, dy);
    println!("slippage: {} vs {}", cp_slippage, slippage);

    println!("\n\nTest 2: 500k Token A, 500k Token B, A = 100, fee_bps = 10, dx = 10k");
    let pool2_reserves = vec![500_000, 500_000];
    let pool2_amp = 100;
    let pool2_fee = 10;
    let dx2 = 10_000;
    let pool2 = StableSwapPool::new(pool2_reserves.clone(), pool2_amp, pool2_fee).unwrap();
    let d2 = pool2.get_d().unwrap();
    let dy2 = pool2.get_dy(0, 1, dx2, pool2.fee_bps).unwrap();
    let slippage2 = pool2.calculate_slippage_bps(0, 1, dx2);
    println!("dy: {}", dy2);
    println!("slippage: {}", slippage2);

    println!("\nComparison with constant product formula:");
    let cp_pool2 = ConstantProductPool::new(pool2_reserves.clone(), pool2_fee).unwrap();
    let cp_dy2 = cp_pool2.get_dy(0, 1, dx2).unwrap();
    let cp_slippage2 = cp_pool2.calculate_slippage_bps(0, 1, dx2);
    println!("dy: {} vs {}", cp_dy2, dy2);
    println!("slippage: {} vs {}", cp_slippage2, slippage2);



    println!("\n\nTest 3: 1M Token A, 200k Token B, A = 50, fee_bps = 4, dx = 50k");
    let pool3_reserves = vec![1_000_000, 200_000];
    let pool3_amp = 50;
    let pool3_fee = 4;
    let dx3 = 50_000;
    let pool3 = StableSwapPool::new(pool3_reserves.clone(), pool3_amp, pool3_fee).unwrap();
    let d3 = pool3.get_d().unwrap();
    let dy3 = pool3.get_dy(0, 1, dx3, pool3.fee_bps).unwrap();
    let slippage3 = pool3.calculate_slippage_bps(0, 1, dx3);
    println!("dy: {}", dy3);
    println!("slippage: {}", slippage3);

    println!("\nComparison with constant product formula:");
    let cp_pool3 = ConstantProductPool::new(pool3_reserves.clone(), pool3_fee).unwrap();
    let cp_dy3 = cp_pool3.get_dy(0, 1, dx3).unwrap();
    let cp_slippage3 = cp_pool3.calculate_slippage_bps(0, 1, dx3);
    println!("dy: {} vs {}", cp_dy3, dy3);
    println!("slippage: {} vs {}", cp_slippage3, slippage3);


    println!("\n\nTest 4: 3-token pool (800k, 1.2M, 1M), A = 200, fee_bps = 8, dx = 25k");
    let pool4_reserves = vec![800_000, 1_200_000, 1_000_000];
    let pool4_amp = 200;
    let pool4_fee = 8;
    let dx4 = 25_000;
    let pool4 = StableSwapPool::new(pool4_reserves.clone(), pool4_amp, pool4_fee).unwrap();
    let d4 = pool4.get_d().unwrap();
    let dy4 = pool4.get_dy(0, 2, dx4, pool4.fee_bps).unwrap();
    let slippage4 = pool4.calculate_slippage_bps(0, 2, dx4);
    println!("dy: {}", dy4);
    println!("slippage: {}", slippage4);

    println!("\nComparison with constant product formula:");
    let cp_pool4 = ConstantProductPool::new(pool4_reserves.clone(), pool4_fee).unwrap();
    let cp_dy4 = cp_pool4.get_dy(0, 2, dx4).unwrap();
    let cp_slippage4 = cp_pool4.calculate_slippage_bps(0, 2, dx4);
    println!("dy: {} vs {}", cp_dy4, dy4);
    println!("slippage: {} vs {}", cp_slippage4, slippage4);
}
