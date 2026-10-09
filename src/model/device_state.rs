use crate::err::*;
use crate::record::*;

#[cfg(feature = "ac-charger")]
use super::ac_charger_state::AcChargerState;
#[cfg(feature = "battery-monitor")]
use super::battery_monitor_state::BatteryMonitorState;
#[cfg(feature = "inverter")]
use super::inverter_state::InverterState;
#[cfg(feature = "solar-charger")]
use super::solar_charger_state::SolarChargerState;
#[cfg(feature = "test-record")]
use super::test_record_state::TestRecordState;
#[cfg(feature = "ve-bus")]
use super::ve_bus_state::VeBusState;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceState {
    #[cfg(feature = "test-record")]
    TestRecord(TestRecordState),
    #[cfg(feature = "solar-charger")]
    SolarCharger(SolarChargerState),
    #[cfg(feature = "ac-charger")]
    AcCharger(AcChargerState),
    #[cfg(feature = "battery-monitor")]
    BatteryMonitor(BatteryMonitorState),
    #[cfg(feature = "inverter")]
    Inverter(InverterState),
    #[cfg(feature = "ve-bus")]
    VeBus(VeBusState),
}

impl DeviceState {
    pub(crate) fn parse(record: &Record) -> Result<Self> {
        // Decrypt ONCE, before the dispatch. `decrypt` is `inline(always)`, so
        // calling it inside each arm emitted a copy of the CTR keystream setup
        // per supported device type -- six of them, in the function a caller
        // then has to place.
        let data = record.decrypt()?;
        let _ = &data;
        match record.record_type() {
            #[cfg(feature = "test-record")]
            RECORD_TYPE_TEST_RECORD => Ok(Self::TestRecord(TestRecordState::parse(&data)?)),
            #[cfg(feature = "solar-charger")]
            RECORD_TYPE_SOLAR_CHARGER => Ok(Self::SolarCharger(SolarChargerState::parse(&data)?)),
            #[cfg(feature = "battery-monitor")]
            RECORD_TYPE_BATTERY_MONITOR => {
                Ok(Self::BatteryMonitor(BatteryMonitorState::parse(&data)?))
            }
            #[cfg(feature = "inverter")]
            RECORD_TYPE_INVERTER => Ok(Self::Inverter(InverterState::parse(&data)?)),
            #[cfg(feature = "ve-bus")]
            RECORD_TYPE_VE_BUS => Ok(Self::VeBus(VeBusState::parse(&data)?)),
            #[cfg(feature = "ac-charger")]
            RECORD_TYPE_AC_CHARGER => Ok(Self::AcCharger(AcChargerState::parse(&data)?)),
            _ => Err(Error::UnsupportedDeviceType(record.record_type())),
        }
    }
}
