use light_enum::LightEnum;


#[derive(LightEnum)]
enum T {
    A(i32),
    B(i32),
    C
}


pub fn main() {

    let old_a = T::A(0);

    let l = old_a.to_light();

    assert!(TLight::A == l);

}

// to see generation:
// cargo install cargo-expand
// cargo expand --bin test