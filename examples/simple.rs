use atsamd21j18a::Peripherals;
use cortex_m::singleton;

use samd_dma::consts::*;
use samd_dma::{DMAController};
use samd_dma::storage::Storage1; 

fn main() {
    let dma_storage: &'static mut Storage1 = singleton!(: Storage1 = Default::default()).unwrap();
    let peri = Peripherals::take().unwrap();
    let mut dma = DMAController::init(peri.DMAC, dma_storage);

    let mut channel = dma.take_channel::<CH0>().unwrap();
    let descriptor = channel.get_first_descriptor();

    return ();
}