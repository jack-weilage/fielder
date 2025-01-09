use fielder::bitfield;

#[test]
fn nav_pvt_flags_one() {
    bitfield! {
        /// From https://content.u-blox.com/sites/default/files/u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf#%5B%7B%22num%22%3A1688%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C59.527%2C719.337%2Cnull%5D
        struct FlagsOne: u8 {
            /// Valid fix (i.e within DOP & accuracy masks)
            GnssFixOk: 0;
            /// Differential corrections were applied
            DiffSoln: 1;

            /// Power saving mode is not active.
            PsmNotActive: 2..4 = 0;
            /// Enabled (an intermediate state before Acquisition state)
            PsmEnabled: 2..4 = 1;
            /// In Acquisition state
            PsmAcquisition: 2..4 = 2;
            /// In Tracking state
            PsmTracking: 2..4 = 3;
            /// In Optimized Tracking state
            PsmPowerOptimizedTracking: 2..4 = 4;
            /// In Inactive state
            PsmInactive: 2..4 = 5;

            /// Heading of vehicle is valid, only set if the receiver is in sensor fusion mode
            HeadVehValid: 5;

            /// No carrier phase range solution
            CarrSolnNone: 6..7 = 0;
            /// Carrier phase range solution with floating ambiguities
            CarrSolnFloating: 6..7 = 1;
            /// Carrier phase range solution with fixed ambiguities
            CarrSolnFixed: 6..7 = 2;
        }
    };

    let one = FlagsOne::from_bits(0b00000001);
    assert!(one.contains(FlagsOne::GnssFixOk));
    assert!(!one.contains(FlagsOne::DiffSoln));
    assert!(one.contains(FlagsOne::PsmNotActive));

    let two = FlagsOne::from_bits(0b00001110);
    assert!(!two.contains(FlagsOne::GnssFixOk));
    assert!(two.contains(FlagsOne::DiffSoln));
    assert!(!two.contains(FlagsOne::PsmEnabled));
    assert!(two.contains(FlagsOne::PsmTracking));

    let three = FlagsOne::from_bits(0)
        .set(FlagsOne::GnssFixOk)
        .set(FlagsOne::DiffSoln)
        .set(FlagsOne::PsmInactive)
        .set(FlagsOne::CarrSolnNone);
    assert_eq!(three.to_bits(), 0b00010111);
}
