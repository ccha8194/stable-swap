use num_traits::pow;


pub struct StableSwapPool {

    pub reserves: Vec<u128>,

    pub amplification_coefficient: u128,

    pub fee_bps: u16,

}


impl StableSwapPool {
    // according to paper,optimal fee: 6 (0.06%), optimal A: 85
    pub fn new(reserves: Vec<u128>, amplification_coefficient: u128, fee_bps: u16) -> Result<Self, SwapError> {
        if reserves.len() < 2 { return Err(SwapError::PoolSizeTooSmall); }
        Ok(Self { 
            reserves, 
            amplification_coefficient, 
            fee_bps 
        })
    }

    // add in dx of token i, return dy of token j
    // i and j are indexes of specified tokens in reserves vec
    pub fn get_dy(&self, i: usize, j: usize, dx: u128, fee_bps: u16) -> Result<u128, SwapError> {
        if i == j { return Err(SwapError::InvalidIndex); }
        if i >= self.reserves.len() || j >= self.reserves.len() { return Err(SwapError::InvalidIndex); }
        if dx == 0 { return Err(SwapError::ZeroAmount); }
    
        let n_coins = self.reserves.len() as u128;
        let amp_times_n_pow_n = self.amplification_coefficient
            .checked_mul(n_coins.pow(n_coins as u32))
            .ok_or(SwapError::MathOverflow)?;
    
        let invariant_d = self.get_d()?;  
    
        let mut sum_excluding_j: u128 = 0;
        let mut c_term: u128 = invariant_d;
    
        for k in 0..self.reserves.len() { // builds C term and sum_excluding_j (to be used in B term)
            if k == j { continue; }
            let x_k = if k == i {
                self.reserves[k].checked_add(dx).ok_or(SwapError::MathOverflow)?
            } else {
                self.reserves[k]
            };
            //if x_k == 0 { return Err(SwapError::InsufficientLiquidity); }
            sum_excluding_j = sum_excluding_j.checked_add(x_k).ok_or(SwapError::MathOverflow)?;
            let num = c_term.checked_mul(invariant_d).ok_or(SwapError::MathOverflow)?; // creates D^n+1 term
            let den = x_k.checked_mul(n_coins).ok_or(SwapError::MathOverflow)?; // creates n^n term and the P term (product of all reserves besides j)
            c_term = num.checked_div(den).ok_or(SwapError::MathOverflow)?;
        }
    
        let c_term = c_term.checked_mul(invariant_d).ok_or(SwapError::MathOverflow)? // finishes C term (An^n part) and final n for the n"n term
            .checked_div(amp_times_n_pow_n).ok_or(SwapError::MathOverflow)?
            .checked_div(n_coins).ok_or(SwapError::MathOverflow)?;
    
        let b_term = sum_excluding_j.checked_add(
            invariant_d.checked_div(amp_times_n_pow_n).ok_or(SwapError::MathOverflow)? // finishes S' + D/An^n
        ).ok_or(SwapError::MathOverflow)?;
    
        let mut y_prev: u128 = 0;
        let mut y: u128 = invariant_d;
    
        for _ in 0..255 { // Newton's method using ynext equation (see ReadMe)
            y_prev = y;
            let numerator = y.checked_mul(y).ok_or(SwapError::MathOverflow)?
                .checked_add(c_term).ok_or(SwapError::MathOverflow)?;
            let denom_left = y.checked_mul(2).ok_or(SwapError::MathOverflow)?
                .checked_add(b_term).ok_or(SwapError::MathOverflow)?;
            let denominator = denom_left.checked_sub(invariant_d).ok_or(SwapError::MathOverflow)?;
            if denominator == 0 { return Err(SwapError::MathOverflow); }
            y = numerator.checked_div(denominator).ok_or(SwapError::MathOverflow)?;
            if y > y_prev {
                if y.checked_sub(y_prev).ok_or(SwapError::MathOverflow)? <= 1 { break; }
            } else if y_prev.checked_sub(y).ok_or(SwapError::MathOverflow)? <= 1 {
                break;
            }
        }
    
        let old_y = self.reserves[j];
        if y > old_y { return Err(SwapError::InsufficientLiquidity); }
        let dy = old_y.checked_sub(y).ok_or(SwapError::MathOverflow)?;


        let fee_den = 10_000u128;
        let fee_num = fee_den
            .checked_sub(self.fee_bps as u128)
            .ok_or(SwapError::MathOverflow)?;
        let dy_net = dy.checked_mul(fee_num).ok_or(SwapError::MathOverflow)?
            .checked_div(fee_den).ok_or(SwapError::MathOverflow)?;

        Ok(dy_net)
    }


    
    pub fn get_d(&self) -> Result<u128, SwapError> {
        let n_coins = self.reserves.len() as u128;
        let amp_times_n_pow_n = self.amplification_coefficient
            .checked_mul(n_coins.pow(n_coins as u32))
            .ok_or(SwapError::MathOverflow)?;
        
        let sum_reserves: u128 = self.reserves.iter().sum();
        /*
        if sum_reserves == 0 {
            return Ok(0);
        }
        */
        
        let mut invariant = sum_reserves;
        let mut previous_invariant;
        for _ in 0..255 {  // newton's method for get_d. see ReadMe for full equation
            let mut d_product = invariant; // start computing product term Dp from current guess of D (Dp = D^(n+1)/n^n*P)
            for reserve in &self.reserves {
                if *reserve == 0 {
                    return Err(SwapError::InsufficientLiquidity);
                }
                d_product = d_product.checked_mul(invariant).ok_or(SwapError::MathOverflow)?;
                d_product = d_product.checked_div(*reserve).ok_or(SwapError::MathOverflow)?;
                d_product = d_product.checked_div(n_coins).ok_or(SwapError::MathOverflow)?;
            }
            
            previous_invariant = invariant;
            
            // calculate numerator = (A*n^n * S + d_product * n) * d
            let term1 = amp_times_n_pow_n.checked_mul(sum_reserves).ok_or(SwapError::MathOverflow)?; // An^n * S
            let term2 = n_coins.checked_mul(d_product).ok_or(SwapError::MathOverflow)?; // n * Dp
            let sum_terms = term1.checked_add(term2).ok_or(SwapError::MathOverflow)?;
            let numerator = sum_terms.checked_mul(invariant).ok_or(SwapError::MathOverflow)?; // (An^n * S + n * Dp) * D
            
            // calculate denominator = (A*n^n - 1) * invariant + (n + 1) * d_product
            if amp_times_n_pow_n <= 1 {
                return Err(SwapError::InvalidIndex);
            }
            let termA = (amp_times_n_pow_n - 1).checked_mul(invariant).ok_or(SwapError::MathOverflow)?; // (An^n - 1) * D
            let termB = (n_coins + 1).checked_mul(d_product).ok_or(SwapError::MathOverflow)?; // (n + 1) * Dp
            let denominator = termA.checked_add(termB).ok_or(SwapError::MathOverflow)?; // (An^n - 1) * D + (n + 1) * Dp
            if denominator == 0 {
                return Err(SwapError::MathOverflow);
            }
            
            // Update Dnext
            invariant = numerator.checked_div(denominator).ok_or(SwapError::MathOverflow)?;
            
            // check convergence (if change is <= 1)
            // a little confusing but can update convergence value later potentially
            if invariant > previous_invariant {
                if invariant - previous_invariant <= 1 {
                    break;
                }
            } else if previous_invariant - invariant <= 1 {
                break;
            }
        }
        
        if invariant == 0 {
            return Err(SwapError::ConvergenceFailed);
        }
        Ok(invariant)
    }

 




    pub fn calculate_slippage_bps(&self, i: usize, j: usize, dx: u128) -> i32 {
        if i == j || i >= self.reserves.len() || j >= self.reserves.len() || dx == 0 {
            return 0;
        }

        let expected: u128 = dx; // peg expectation

        // actual output (not including any fee)
        let actual = match self.get_dy(i, j, dx, 0) {
            Ok(v) => v,
            Err(_) => return 0,
        };

        // diff = expected - actual (signed this one)
        let expected_i = expected as i128;
        let actual_i   = actual as i128;
        let diff: i128 = expected_i - actual_i;

        let scale: i128 = 10_000; 
        let num = diff.saturating_mul(scale);
        let den = expected_i;
        if den == 0 {
            return 0;
        }

        let half = den / 2;
        let rounded = if num >= 0 {
            (num + half) / den
        } else {
            (num - half) / den
        };

        rounded.clamp(i32::MIN as i128, i32::MAX as i128) as i32
    }

}





#[derive(Debug)]
pub enum SwapError {
    InvalidIndex,
    ZeroAmount,
    MathOverflow,
    InsufficientLiquidity,
    ConvergenceFailed,
    PoolSizeTooSmall,
}
