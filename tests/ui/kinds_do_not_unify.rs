uuid_macros::typed_uuid!(RegionId, "region");
uuid_macros::typed_uuid!(ZoneId, "zone");

fn takes_region(_: RegionId) {}

fn main() {
    takes_region(ZoneId::new_v4());
    let _: RegionId = ZoneId::named("x");
}
