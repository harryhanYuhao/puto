use puto;

fn main() {
    let (i, (j, k)) = puto::eular_cal_coefficient(120, 180);
    println!("{:?}, ", (i, (j, k)));
}
