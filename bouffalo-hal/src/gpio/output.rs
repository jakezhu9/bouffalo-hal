use super::{
    alternate::Alternate,
    convert::{IntoPad, IntoPadv2},
    input::Input,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::{self, Drive, RegisterBlock};
use core::ops::Deref;
use embedded_hal::digital::{ErrorType, OutputPin};

/// GPIO pad in output mode.
pub struct Output<GLB, const N: usize, M> {
    inner: super::Inner<GLB, N, typestate::Output<M>>,
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Output<GLB, N, M> {
    /// Get drive strength of this pad.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.inner.drive()
    }
    /// Set drive strength of this pad.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        self.inner.set_drive(val)
    }
}

impl<GLB: Deref<Target = RegisterBlock>, const N: usize, M> IntoPad<GLB, N> for Output<GLB, N, M> {
    #[inline]
    fn into_pull_up_output(self) -> Output<GLB, N, PullUp> {
        self.inner.into_pull_up_output().into()
    }
    #[inline]
    fn into_pull_down_output(self) -> Output<GLB, N, PullDown> {
        self.inner.into_pull_down_output().into()
    }
    #[inline]
    fn into_floating_output(self) -> Output<GLB, N, Floating> {
        self.inner.into_floating_output().into()
    }
    #[inline]
    fn into_pull_up_input(self) -> Input<GLB, N, PullUp> {
        self.inner.into_pull_up_input().into()
    }
    #[inline]
    fn into_pull_down_input(self) -> Input<GLB, N, PullDown> {
        self.inner.into_pull_down_input().into()
    }
    #[inline]
    fn into_floating_input(self) -> Input<GLB, N, Floating> {
        self.inner.into_floating_input().into()
    }
}

#[cfg(any(doc, feature = "glb-v2"))]
impl<GLB: Deref<Target = RegisterBlock>, const N: usize, M> IntoPadv2<GLB, N>
    for Output<GLB, N, M>
{
    #[inline]
    fn into_spi<const I: usize>(self) -> Alternate<GLB, N, typestate::Spi<I>> {
        self.inner.into_spi().into()
    }
    #[inline]
    fn into_sdh(self) -> Alternate<GLB, N, typestate::Sdh> {
        self.inner.into_sdh().into()
    }
    #[inline]
    fn into_uart(self) -> Alternate<GLB, N, typestate::Uart> {
        self.inner.into_uart().into()
    }
    #[inline]
    fn into_mm_uart(self) -> Alternate<GLB, N, typestate::MmUart> {
        self.inner.into_mm_uart().into()
    }
    #[inline]
    fn into_pull_up_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>> {
        self.inner.into_pull_up_pwm().into()
    }
    #[inline]
    fn into_pull_down_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>> {
        self.inner.into_pull_down_pwm().into()
    }
    #[inline]
    fn into_floating_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>> {
        self.inner.into_floating_pwm().into()
    }
    #[inline]
    fn into_i2c<const I: usize>(self) -> Alternate<GLB, N, typestate::I2c<I>> {
        self.inner.into_i2c().into()
    }
    #[inline]
    fn into_jtag_d0(self) -> Alternate<GLB, N, typestate::JtagD0> {
        self.inner.into_jtag_d0().into()
    }
    #[inline]
    fn into_jtag_m0(self) -> Alternate<GLB, N, typestate::JtagM0> {
        self.inner.into_jtag_m0().into()
    }
    #[inline]
    fn into_jtag_lp(self) -> Alternate<GLB, N, typestate::JtagLp> {
        self.inner.into_jtag_lp().into()
    }
}

impl<GLB, const N: usize, M> ErrorType for Output<GLB, N, M> {
    type Error = core::convert::Infallible;
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> OutputPin for Output<GLB, N, M> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.inner.set_low()
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.inner.set_high()
    }
}

// This part of implementation using `embedded_hal_027` is designed for backward compatibility of
// ecosystem crates, as some of them depends on embedded-hal v0.2.7 traits.
// We encourage ecosystem developers to use embedded-hal v1.0.0 traits; after that, this part of code
// would be removed in the future.
impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M>
    embedded_hal_027::digital::v2::OutputPin for Output<GLB, N, M>
{
    type Error = core::convert::Infallible;
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        <Self as OutputPin>::set_low(self)
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        <Self as OutputPin>::set_high(self)
    }
}

impl<GLB, const N: usize, M> From<super::Inner<GLB, N, typestate::Output<M>>>
    for Output<GLB, N, M>
{
    #[inline]
    fn from(inner: super::Inner<GLB, N, typestate::Output<M>>) -> Self {
        Self { inner }
    }
}
