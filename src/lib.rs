#![no_std]

mod traits;
pub use traits::*;

#[macro_export]
macro_rules! bitfield {
    (
        $(#[$outer:meta])*
        $vis:vis struct $struct:ident: $ty:ty {
            $($tt:tt)*
        }
    ) => {
        $(#[$outer])*
        $vis struct $struct($ty);

        #[allow(non_upper_case_globals)]
        impl $struct {
            bitfield!(@consts $($tt)*);
        }
    };
    (@consts) => {};
    (@consts $flag:tt:$bit:tt; $($tt:tt)*) => {
        const $flag: u8 = $bit;
        bitfield!(@consts $($tt)*);
    };
    (@consts $flag:tt:$bit:tt = $value:tt; $($tt:tt)*) => {
        const $flag: (u8, u8) = ($bit, $value);
        bitfield!(@consts $($tt)*);
    };
    (@consts $flag:tt:$msb:tt..$lsb:tt = $value:tt; $($tt:tt)*) => {
        const $flag: (u8, u8, u8) = ($msb, $lsb, $value);
        bitfield!(@consts $($tt)*);
    };
}

#[cfg(test)]
mod tests {
    bitfield! {
        // From https://content.u-blox.com/sites/default/files/u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf#%5B%7B%22num%22%3A1688%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C59.527%2C719.337%2Cnull%5D
        struct FlagsOne: u8 {
            GnssFixOk: 0;
            DiffSoln: 1;

            PsmNotActive: 2..4 = 0;
            PsmEnabled: 2..4 = 1;
            PsmAcquisition: 2..4 = 2;
            PsmTracking: 2..4 = 3;
            PsmPowerOptimizedTracking: 2..4 = 4;
            PsmInactive: 2..4 = 5;

            HeadVehValid: 5;
            CarrSolnNone: 6..7 = 0;
            CarrSolnFloating: 6..7 = 1;
            CarrSolnFixed: 6..7 = 2;
        }
    }
}
