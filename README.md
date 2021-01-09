![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# samd-dma

## Usage

Add the following line to your dependencies section in `Cargo.toml`

```toml
samd-dma = "0.3.0"
```

The following feature flags control which MCU variant you are targeting.

| Name (Docs) | # DMA Channels | Boards |
|:------|:----|:-----------------------|
| [samd21g18a](https://proman21.github.io/samd-dma/samd21g18a/index.html) | 12 | Circuit Playground Express, Feather M0, Metro M0, MKR ZERO, SAMD21 Mini, SODAQ ONE |
| [samd21e18a](https://proman21.github.io/samd-dma/samd21e18a/index.html) | 12 | Gemma M0, Trinket M0, Serpente |
| [samd21j18a](https://proman21.github.io/samd-dma/samd21j18a/index.html) | 12 | SODAQ SARA AFF |
| [samd51j19a](https://proman21.github.io/samd-dma/samd51j19a/index.html) | 32 | EdgeBadge, Feather M4, Metro M4 |
| [samd51j20a](https://proman21.github.io/samd-dma/samd51j20a/index.html) | 32 | PyPortal |
| [samd51g19a](https://proman21.github.io/samd-dma/samd51g19a/index.html) | 32 | ItsyBitsy M4, Trellis M4 |

## About

DMA library for Microchip SAM micro-controllers.

This library provides a convenience wrapper around the DMA and CRC subsystem of the SAM family of micro-controllers.
It is designed to be maximally expressive while providing a convenient interface for simple tasks.

## Safety

The primary goal of this library is to abstract away directly writing to registers. It is not concerned with larger
goals like totally memory safe DMA, which is left to the discretion of the library user and the goals you are
trying to accomplish. If you only need one descriptor per channel and aren't reading from the write-back address,
you can use this library without any unsafe sections. More advanced features of the DMA system are not protected by
this library, and it is possible to shot yourself in the foot if not careful. I highly recommend reading the
relevant sections of the manual for your family of micro-controller to understand how the DMA system works and
operates on memory outside of the compilers knowledge.

## SAMD21

Because of the design of the DMA system on the SAMD21 family, any channel methods that modify channel registers are
NOT interrupt-safe. Beware of accessing or mutating channel register without calling in an interrupt-free section.

## License

`samd-dma` is distributed under the MIT license. The full text of the license can be found in `LICENSE` file.
