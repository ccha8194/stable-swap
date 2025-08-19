use crate::stableSwapPool::SwapError;

pub struct ConstantProductPool {
    pub reserves: Vec<u128>,
    pub fee_bps: u16,
}

impl ConstantProductPool {
    pub fn new(reserves: Vec<u128>, fee_bps: u16) -> Result<Self, SwapError> {
        if reserves.len() < 2 {
            return Err(SwapError::PoolSizeTooSmall);
        }
        Ok(Self { reserves, fee_bps })
    }

    pub fn get_dy(&self, i: usize, j: usize, dx: u128) -> Result<u128, SwapError> {
        if i == j { return Err(SwapError::InvalidIndex); }
        if i >= self.reserves.len() || j >= self.reserves.len() { return Err(SwapError::InvalidIndex); }
        if dx == 0 { return Err(SwapError::ZeroAmount); }

        let x = self.reserves[i];
        let y = self.reserves[j];

        let new_x = x.checked_add(dx).ok_or(SwapError::MathOverflow)?;
        if new_x == 0 {
            return Err(SwapError::MathOverflow);
        }

        let dy = y.checked_mul(dx).ok_or(SwapError::MathOverflow)?
            .checked_div(new_x).ok_or(SwapError::MathOverflow)?;

        let fee_den = 10_000u128;
        let fee_num = fee_den.checked_sub(self.fee_bps as u128).ok_or(SwapError::MathOverflow)?;
        let dy_net = dy.checked_mul(fee_num).ok_or(SwapError::MathOverflow)?
            .checked_div(fee_den).ok_or(SwapError::MathOverflow)?;

        Ok(dy_net)
    }

    pub fn calculate_slippage_bps(&self, i: usize, j: usize, dx: u128) -> i32 {
        if i == j || i >= self.reserves.len() || j >= self.reserves.len() || dx == 0 {
            return 0;
        }

        let expected_dy = dx as i128;

        let actual_dy = self.get_dy(i, j, dx).unwrap_or(0) as i128;

        let diff = expected_dy - actual_dy;

        let scale: i128 = 10_000;
        let num = diff.saturating_mul(scale);
        let den = expected_dy;

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
