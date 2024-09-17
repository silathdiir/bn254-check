use ark_bn254::Bn254 as ArkBn254;
use ark_bn254::Fq as ArkFq;
use ark_bn254::Fq12 as ArkFq12;
use ark_bn254::Fq2 as ArkFq2;
use ark_bn254::Fq6 as ArkFq6;
use ark_bn254::G1Affine as ArkG1;
use ark_bn254::G2Affine as ArkG2;
use ark_ec::bn::BnConfig;
use ark_ec::pairing::Pairing;
use ark_ec::pairing::PairingOutput;
use ark_ec::AffineRepr;
use ark_ff::BigInteger;
use ark_ff::Fp;
use ark_ff::MontFp;
use ark_ff::QuadExtField;
use halo2_proofs::halo2curves::bn256::pairing;
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr, G1Affine as HaloG1, G2Affine as HaloG2},
    plonk::{self, ProvingKey},
    poly::{commitment::Params, kzg::commitment::ParamsKZG},
};
use num_bigint::BigInt;
use num_bigint::BigUint;
use plonky2_bn254_pairing::miller_loop_native::miller_loop_native;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let halo_g1 = HaloG1::generator();
    let halo_g2 = HaloG2::generator();
    println!("test - halo_g1 = {halo_g1:#?}");
    println!("test - halo_g2 = {halo_g2:#?}");
    println!("\n\n");

    let ark_g1 = ArkG1::generator();
    let ark_g2 = ArkG2::generator();
    println!("test - ark_g1 = {ark_g1:#?}");
    println!("test - ark_g2 = {ark_g2:#?}");
    println!("\n\n");

    let halo_pairing = pairing(&halo_g1, &halo_g2);
    println!("test - halo2_pairing = {:?}", halo_pairing);
    println!("\n\n");

    let ark_pairing: PairingOutput<_> = ArkBn254::pairing(ark_g1, ark_g2);
    let b: ArkFq12 = ark_pairing.0.into();
    println!("test - ark_pairing = {:?}", b);
}
