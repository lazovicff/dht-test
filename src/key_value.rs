use halo2wrong::{curves::bn256::Fr, halo2::arithmetic::Field};
use rand::RngCore;

pub struct DHTKey {
	x: Fr,
	y: Fr,
}

impl DHTKey {
	pub fn new(x: Fr, y: Fr) -> DHTKey {
		Self { x, y }
	}

	pub fn random<R: RngCore + Clone>(rng: &mut R) -> DHTKey {
		let x = Fr::random(rng.clone());
		let y = Fr::random(rng);
		DHTKey { x, y }
	}

	pub fn from_bytes(mut bytes: Vec<u8>) -> DHTKey {
		let x_drained: Vec<u8> = bytes.drain(..32).collect();
		let y_drained: Vec<u8> = bytes.drain(..32).collect();

		let mut x_bytes = [0u8; 32];
		x_bytes.copy_from_slice(&x_drained);
		let mut y_bytes = [0u8; 32];
		y_bytes.copy_from_slice(&y_drained);

		DHTKey {
			x: Fr::from_bytes(&x_bytes).unwrap(),
			y: Fr::from_bytes(&y_bytes).unwrap(),
		}
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.extend_from_slice(&self.x.to_bytes());
		bytes.extend_from_slice(&self.y.to_bytes());
		bytes
	}
}

pub struct DHTValue {
	sig_r_x: Fr,
	sig_r_y: Fr,
	sig_s: Fr,
	neighbours: [Fr; 2],
	scores: [u32; 2],
}

impl DHTValue {
	pub fn new(sig_r_x: Fr, sig_r_y: Fr, sig_s: Fr, neighbours: [Fr; 2], scores: [u32; 2]) -> Self {
		Self { sig_r_x, sig_r_y, sig_s, neighbours, scores }
	}

	pub fn random<R: RngCore + Clone>(rng: &mut R) -> DHTValue {
		let sig_r_x: Fr = Fr::random(rng.clone());
		let sig_r_y: Fr = Fr::random(rng.clone());
		let sig_s: Fr = Fr::random(rng.clone());
		let neighbours: [Fr; 2] = [(); 2].map(|_| Fr::random(rng.clone()));
		let scores: [u32; 2] = [(); 2].map(|_| rng.next_u32());
		DHTValue { sig_r_x, sig_r_y, sig_s, neighbours, scores }
	}

	pub fn from_bytes(mut bytes: Vec<u8>) -> DHTValue {
		let sig_r_x_drained: Vec<u8> = bytes.drain(..32).collect();
		let sig_r_y_drained: Vec<u8> = bytes.drain(..32).collect();
		let sig_s_drained: Vec<u8> = bytes.drain(..32).collect();
		let mut neighbours: [Vec<u8>; 2] = [(); 2].map(|_| Vec::new());
		let mut scores: [Vec<u8>; 2] = [(); 2].map(|_| Vec::new());
		for i in 0..2 {
			neighbours[i] = bytes.drain(..32).collect();
		}
		for i in 0..2 {
			scores[i] = bytes.drain(..4).collect();
		}

		let mut sig_r_x_bytes = [0u8; 32];
		sig_r_x_bytes.copy_from_slice(&sig_r_x_drained);
		let mut sig_r_y_bytes = [0u8; 32];
		sig_r_y_bytes.copy_from_slice(&sig_r_y_drained);
		let mut sig_s_bytes = [0u8; 32];
		sig_s_bytes.copy_from_slice(&sig_s_drained);
		let neighbours_bytes = neighbours.map(|x| {
			let mut b = [0u8; 32];
			b.copy_from_slice(&x);
			b
		});
		let score_bytes = scores.map(|x| {
			let mut b = [0u8; 4];
			b.copy_from_slice(&x);
			b
		});

		DHTValue {
			sig_r_x: Fr::from_bytes(&sig_r_x_bytes).unwrap(),
			sig_r_y: Fr::from_bytes(&sig_r_y_bytes).unwrap(),
			sig_s: Fr::from_bytes(&sig_s_bytes).unwrap(),
			neighbours: neighbours_bytes.map(|x| Fr::from_bytes(&x).unwrap()),
			scores: score_bytes.map(|x| u32::from_be_bytes(x)),
		}
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.extend_from_slice(&self.sig_r_x.to_bytes());
		bytes.extend_from_slice(&self.sig_r_y.to_bytes());
		bytes.extend_from_slice(&self.sig_s.to_bytes());
		for i in 0..2 {
			bytes.extend_from_slice(&self.neighbours[i].to_bytes());
		}
		for i in 0..2 {
			bytes.extend_from_slice(&self.scores[i].to_be_bytes());
		}
		bytes
	}
}