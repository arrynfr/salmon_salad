/*
		spi@23510c000 {
			compatible = "apple,t8103-spi\0apple,spi";
			reg = <0x02 0x3510c000 0x00 0x4000>;
			interrupt-parent = <0x0f>;
			interrupts = <0x00 0x269 0x04>;
			clocks = <0x4d>;
			pinctrl-0 = <0x4e>;
			pinctrl-names = "default";
			power-domains = <0x4f>;
			#address-cells = <0x01>;
			#size-cells = <0x00>;
			status = "okay";

			hid-transport@0 {
				compatible = "apple,spi-hid-transport";
				reg = <0x00>;
				spi-max-frequency = <0x7a1200>;
				spi-cs-setup-delay-ns = <0xfde8>;
				spi-cs-hold-delay-ns = <0xfde8>;
				spi-cs-inactive-delay-ns = <0x3d090>;
				spien-gpios = <0x39 0xc3 0x00>;
				interrupts-extended = <0x45 0x0d 0x08>;
			};
		};
*/

/*
#define APPLE_SPI_CTRL			0x000
#define APPLE_SPI_CTRL_RUN		BIT(0)
#define APPLE_SPI_CTRL_TX_RESET		BIT(2)
#define APPLE_SPI_CTRL_RX_RESET		BIT(3)

#define APPLE_SPI_CFG			0x004
#define APPLE_SPI_CFG_CPHA		BIT(1)
#define APPLE_SPI_CFG_CPOL		BIT(2)
#define APPLE_SPI_CFG_MODE		GENMASK(6, 5)
#define APPLE_SPI_CFG_MODE_POLLED	0
#define APPLE_SPI_CFG_MODE_IRQ		1
#define APPLE_SPI_CFG_MODE_DMA		2
#define APPLE_SPI_CFG_IE_RXCOMPLETE	BIT(7)
#define APPLE_SPI_CFG_IE_TXRXTHRESH	BIT(8)
#define APPLE_SPI_CFG_LSB_FIRST		BIT(13)
#define APPLE_SPI_CFG_WORD_SIZE		GENMASK(16, 15)
#define APPLE_SPI_CFG_WORD_SIZE_8B	0
#define APPLE_SPI_CFG_WORD_SIZE_16B	1
#define APPLE_SPI_CFG_WORD_SIZE_32B	2
#define APPLE_SPI_CFG_FIFO_THRESH	GENMASK(18, 17)
#define APPLE_SPI_CFG_FIFO_THRESH_8B	0
#define APPLE_SPI_CFG_FIFO_THRESH_4B	1
#define APPLE_SPI_CFG_FIFO_THRESH_1B	2
#define APPLE_SPI_CFG_IE_TXCOMPLETE	BIT(21)

#define APPLE_SPI_STATUS		0x008
#define APPLE_SPI_STATUS_RXCOMPLETE	BIT(0)
#define APPLE_SPI_STATUS_TXRXTHRESH	BIT(1)
#define APPLE_SPI_STATUS_TXCOMPLETE	BIT(2)


#define APPLE_SPI_PIN_KEEP_MOSI		BIT(0)
#define APPLE_SPI_PIN_CS		BIT(1)

#define APPLE_SPI_TXDATA		0x010
#define APPLE_SPI_RXDATA		0x020
#define APPLE_SPI_CLKDIV		0x030
#define APPLE_SPI_CLKDIV_MIN		0x002
#define APPLE_SPI_CLKDIV_MAX		0x7ff
#define APPLE_SPI_RXCNT			0x034
#define APPLE_SPI_WORD_DELAY		0x038
#define APPLE_SPI_TXCNT			0x04c

#define APPLE_SPI_FIFOSTAT		0x10c
#define APPLE_SPI_FIFOSTAT_TXFULL	BIT(4)
#define APPLE_SPI_FIFOSTAT_LEVEL_TX	GENMASK(15, 8)
#define APPLE_SPI_FIFOSTAT_RXEMPTY	BIT(20)
#define APPLE_SPI_FIFOSTAT_LEVEL_RX	GENMASK(31, 24)

#define APPLE_SPI_IE_XFER		0x130
#define APPLE_SPI_IF_XFER		0x134
#define APPLE_SPI_XFER_RXCOMPLETE	BIT(0)
#define APPLE_SPI_XFER_TXCOMPLETE	BIT(1)

#define APPLE_SPI_IE_FIFO		0x138
#define APPLE_SPI_IF_FIFO		0x13c
#define APPLE_SPI_FIFO_RXTHRESH		BIT(4)
#define APPLE_SPI_FIFO_TXTHRESH		BIT(5)
#define APPLE_SPI_FIFO_RXFULL		BIT(8)
#define APPLE_SPI_FIFO_TXEMPTY		BIT(9)
#define APPLE_SPI_FIFO_RXUNDERRUN	BIT(16)
#define APPLE_SPI_FIFO_TXOVERFLOW	BIT(17)

#define APPLE_SPI_SHIFTCFG		0x150
#define APPLE_SPI_SHIFTCFG_CLK_ENABLE	BIT(0)
#define APPLE_SPI_SHIFTCFG_CS_ENABLE	BIT(1)
#define APPLE_SPI_SHIFTCFG_AND_CLK_DATA	BIT(8)
#define APPLE_SPI_SHIFTCFG_CS_AS_DATA	BIT(9)
#define APPLE_SPI_SHIFTCFG_TX_ENABLE	BIT(10)
#define APPLE_SPI_SHIFTCFG_RX_ENABLE	BIT(11)
#define APPLE_SPI_SHIFTCFG_BITS		GENMASK(21, 16)
#define APPLE_SPI_SHIFTCFG_OVERRIDE_CS	BIT(24)

#define APPLE_SPI_PINCFG		0x154
#define APPLE_SPI_PINCFG_KEEP_CLK	BIT(0)
#define APPLE_SPI_PINCFG_KEEP_CS	BIT(1)
#define APPLE_SPI_PINCFG_KEEP_MOSI	BIT(2)
#define APPLE_SPI_PINCFG_CLK_IDLE_VAL	BIT(8)
#define APPLE_SPI_PINCFG_CS_IDLE_VAL	BIT(9)
#define APPLE_SPI_PINCFG_MOSI_IDLE_VAL	BIT(10)

#define APPLE_SPI_DELAY_PRE		0x160
#define APPLE_SPI_DELAY_POST		0x168
#define APPLE_SPI_DELAY_ENABLE		BIT(0)
#define APPLE_SPI_DELAY_NO_INTERBYTE	BIT(1)
#define APPLE_SPI_DELAY_SET_SCK		BIT(4)
#define APPLE_SPI_DELAY_SET_MOSI	BIT(6)
#define APPLE_SPI_DELAY_SCK_VAL		BIT(8)
#define APPLE_SPI_DELAY_MOSI_VAL	BIT(12)

#define APPLE_SPI_FIFO_DEPTH		16

#define APPLE_SPI_TIMEOUT_MS		200
*/

const SPI_ADDR: *mut u8 = 0x23510c000 as *mut u8;
const SPI_CLK_FRQ: u32 = 0x7270e00;
const APPLE_SPI_PIN: usize = 0x00c;
const APPLE_SPI_PIN_CS: u32 = 0b1;
const APPLE_SPI_SHIFTCFG: usize = 0x150;
const APPLE_SPI_SHIFTCFG_OVERRIDE_CS: u32 = 0b1 << 24;
const APPLE_SPI_PINCFG: usize = 0x154;
const APPLE_SPI_PINCFG_CS_IDLE_VAL: u32 = 0b1 << 9;
const APPLE_SPI_PINCFG_KEEP_CS: u32 = 0b1;
const APPLE_SPI_CTRL: usize = 0x000;
const APPLE_SPI_CTRL_TX_RESET: u32 = 0b1 << 2;
const APPLE_SPI_CTRL_RX_RESET: u32 = 0b1 << 3;
const APPLE_SPI_CFG_MODE_POLLED: u32 = 0;
const APPLE_SPI_CFG: usize = 0x004;
const APPLE_SPI_CFG_WORD_SIZE_8B: u32 = 0;
const GPIO: *mut u8 = 0x23c100000 as *mut u8;
const GPIO_SPI_EN: usize = 0xc3;

pub fn testspi() {
    unsafe {
        (SPI_ADDR.add(APPLE_SPI_PIN) as *mut u32).write_volatile(APPLE_SPI_PIN_CS);
        let mut sm = (SPI_ADDR.add(APPLE_SPI_SHIFTCFG) as *mut u32).read_volatile();
        sm &= !APPLE_SPI_SHIFTCFG_OVERRIDE_CS;
        (SPI_ADDR.add(APPLE_SPI_SHIFTCFG) as *mut u32).write_volatile(sm);
        let mut pc = (SPI_ADDR.add(APPLE_SPI_PINCFG) as *mut u32).read_volatile();
        pc &= !APPLE_SPI_PINCFG_CS_IDLE_VAL | APPLE_SPI_PINCFG_KEEP_CS;
        (SPI_ADDR.add(APPLE_SPI_PINCFG) as *mut u32).write_volatile(pc);

        
        (SPI_ADDR.add(APPLE_SPI_CTRL) as *mut u32).write_volatile(APPLE_SPI_CTRL_TX_RESET | APPLE_SPI_CTRL_RX_RESET);

        	/* Configure defaults */
	    /*writel(FIELD_PREP(APPLE_SPI_CFG_MODE, APPLE_SPI_CFG_MODE_IRQ) |
        FIELD_PREP(APPLE_SPI_CFG_WORD_SIZE, APPLE_SPI_CFG_WORD_SIZE_8B) |
        FIELD_PREP(APPLE_SPI_CFG_FIFO_THRESH, APPLE_SPI_CFG_FIFO_THRESH_8B),
        priv->base + APPLE_SPI_CFG);*/

        (SPI_ADDR.add(APPLE_SPI_CFG) as *mut u32).write_volatile(0);
        (GPIO.add(GPIO_SPI_EN) as *mut u32).write_volatile(0xffffffff);
        }
}

/*
			spi3-pins {
				pinmux = <0x1002e 0x1002f 0x10030 0x10031>;
				phandle = <0x4e>;
			};
*/