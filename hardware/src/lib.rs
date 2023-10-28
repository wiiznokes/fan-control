
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;



trait HardwareSpec {

    fn hardware_type(&self) -> HardwareType;

    fn hardware_id(&self) -> Option<u32>;


}




trait HardwareGenerator {

    fn new() -> impl HardwareGenerator where Self: Sized;

    fn temps<'a>(&'a self) -> Vec<Temp<'a>>;

    fn id<T: HardwareSpec>();

    fn value<T: HardwareSpec>(&item: T) -> Option<i32>;

    fn set_value<T: HardwareSpec>(&item: T, value: i32);

}