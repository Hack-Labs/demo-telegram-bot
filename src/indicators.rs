pub struct SMA {
    pub sequence: Vec<f64>
}


pub struct MAE {
    pub upper_sequence: Vec<f64>,
    pub lower_sequence: Vec<f64>
}




impl SMA {
    pub fn new(price_sequence: &Vec<f64>, window_size: usize) -> SMA {
	let mut sma_sequence: Vec<f64> = Vec::new();
	let mut window_start = 0;

	while window_start + window_size <= price_sequence.len() {
            let window_end = window_start + window_size;
            let sequence_slice = &price_sequence[window_start..window_end];
            let sum: f64 = sequence_slice.iter().sum();
            let average = sum / window_size as f64;

            sma_sequence.push(average);
            window_start += 1;
	}

	SMA { sequence: sma_sequence }
    }
}



impl MAE {
    pub fn new(sma_sequence: &Vec<f64>, envelop_offset: f64) -> MAE {
	let mut upper: Vec<f64> = vec![];
	let mut lower: Vec<f64> = vec![];

	for sma in sma_sequence {
	    upper.push(sma + (sma * envelop_offset));
	    lower.push(sma - (sma * envelop_offset));
	}

	MAE { upper_sequence: upper, lower_sequence: lower }
    }
}
