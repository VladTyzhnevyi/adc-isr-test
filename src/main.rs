//https://rust-embedded.github.io/book/intro/no-std.html
#![feature(const_fn)]
#![no_std] // is a crate-level attribute that indicates that the crate will link to the core-crate instead of the std-crate
#![no_main]

//extern crate libm;
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

#[macro_use(interrupt)]
extern crate tm4c129x;
//use cortex_m::peripheral::syst::SystClkSource;
//use cortex_m::Peripherals;
use cortex_m_rt::{entry, exception}; //limit use of cortex_m_rt to [entry] and [exception]
//use cortex_m_semihosting::hprint;
//extern crate smoltcp;
//extern crate crc;

use core::cell::{Cell, RefCell};
use core::fmt;
use cortex_m::interrupt::Mutex;

//const sys_tick_reload_value: u32 = 12_000_000; //
//const sys_tick_reload_value: u32 = 24_000_000; //
//const sys_tick_reload_value: u32 = 23999; //
//const sys_tick_reload_value: u32 = 24000; //2.5kHz
//const sys_tick_reload_value: u32 = 48000; //1.25kHz
//const sys_tick_reload_value: u32 = 120000; //500Hz
//const sys_tick_reload_value: u32 = 1200000; //500Hz
//const sys_tick_reload_value: u32 = 12_000_000; //5Hz


//const sys_tick_reload_value: u32 = 15_000_000; //4Hz

const SYS_TICK_RELOAD_VALUE: u32 = 15_000_000; //4Hz

static SYS_TICK_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
//create static variables for seconds, minutes, and hours
static SEC_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static MIN_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static HOURS_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

static ADC_IRQ_COUNT: Mutex<Cell<u64>> = Mutex::new(Cell::new(0));

static IC_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
static FBI_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
static FV_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0)); 
static FD_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0)); 
static AV_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0)); 
static FBV_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        write!($crate::UART0, $($arg)*).unwrap()
    })
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[no_mangle] // https://github.com/rust-lang/rust/issues/{38281,51647}
#[panic_handler]
pub fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[macro_use]
mod board;

pub struct UART0;

impl fmt::Write for UART0 {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let uart_0 = unsafe { &*tm4c129x::UART0::ptr() };
        for c in s.bytes() {
            while uart_0.fr.read().txff().bit() {}
            uart_0.dr.write(|w| w.data().bits(c))
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    board::init();
    board::start_sys_tick(SYS_TICK_RELOAD_VALUE);  
    board::start_adc();  
    loop {
       
       /*
        let adc0 = unsafe { &*tm4c129x::ADC0::ptr() };
        if !adc0.ostat.read().ov0().bit() {
            let mut cp = unsafe { tm4c129x::CorePeripherals::steal() };
            cp.NVIC.enable(tm4c129x::Interrupt::ADC0SS0);
        }
        */
        /*
        cortex_m::interrupt::free(|cs| { 
        let ic_sample  = IC_SAMPLE.borrow(cs);
        let fbi_sample = FBI_SAMPLE.borrow(cs);
        let fv_sample  = FV_SAMPLE.borrow(cs);
        let fd_sample  = FD_SAMPLE.borrow(cs);
        let av_sample  = AV_SAMPLE.borrow(cs);
        let fbv_sample = FBV_SAMPLE.borrow(cs);

        println!("ic_sample {}\r", ic_sample.get());        
        println!("fbi_sample {}\r", fbi_sample.get());        
        println!("fv_sample {}\r", fv_sample.get());
        println!("fd_sample {}\r", fd_sample.get());
        println!("av_sample {}\r", av_sample.get());
        println!("fbv_sample {}\r", fbv_sample.get());
        println!("\n");
        });

        */
    }
}


//https://docs.rs/cortex-m-rt/0.6.12/cortex_m_rt/attr.exception.html
#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {        
        let sys_tick_irq_count = SYS_TICK_IRQ_COUNT.borrow(cs);
        let sec_irq_count = SEC_IRQ_COUNT.borrow(cs);
        let min_irq_count = MIN_IRQ_COUNT.borrow(cs);
        let hours_irq_count = HOURS_IRQ_COUNT.borrow(cs);
        sys_tick_irq_count.set(sys_tick_irq_count.get() + 1);
        if sys_tick_irq_count.get()==4{
            
            //define ADC variables
            let ic_sample  = IC_SAMPLE.borrow(cs);
            let fbi_sample = FBI_SAMPLE.borrow(cs);
            let fv_sample  = FV_SAMPLE.borrow(cs);
            let fd_sample  = FD_SAMPLE.borrow(cs);
            let av_sample  = AV_SAMPLE.borrow(cs);
            let fbv_sample = FBV_SAMPLE.borrow(cs);
            //println!("{} {} {} {} {}\r", ic_sample.get(), fbi_sample.get(), fv_sample.get(), fd_sample.get(), av_sample.get());        
            
            //with current UART settings and ADC CLK set to 8 Mhz only 4 values can be rpinted out without ADC FIFO overflow
            println!("{} {} {} {}\r", ic_sample.get(), fbi_sample.get(), fv_sample.get(), fd_sample.get());        
            
            let adc0 = unsafe { &*tm4c129x::ADC0::ptr() };
            
            //if !adc0.ostat.read().ov0().bit() {
                let mut cp = unsafe { tm4c129x::CorePeripherals::steal() };
                cp.NVIC.enable(tm4c129x::Interrupt::ADC0SS0);
            //}

            let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };
            //gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x01));
            gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x01));
            
            //gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x02));


            let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };            
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x40));
            sys_tick_irq_count.set(0);
            sec_irq_count.set(sec_irq_count.get() + 1);
            if sec_irq_count.get() == 59 {
                sec_irq_count.set(0);                
                min_irq_count.set(min_irq_count.get() + 1);
                if min_irq_count.get() == 59 {
                    min_irq_count.set(0);                
                    hours_irq_count.set(hours_irq_count.get() + 1);
                }
            }
            //println!("{:02}:{:02}\r", min_irq_count.get(), sec_irq_count.get());
            //print!(" {:02}:{:02}:{:02}\r", hours_irq_count.get(), min_irq_count.get(), sec_irq_count.get());
        }
        
    })
}

interrupt!(ADC0SS0, adc0_ss0);
fn adc0_ss0() {
    cortex_m::interrupt::free(|cs| {        
        let adc0 = unsafe { &*tm4c129x::ADC0::ptr() };     
        if adc0.ostat.read().ov0().bit() {
            panic!("ADC FIFO overflowed")        
        }
        adc0.isc.write(|w| w.in0().bit(true));    

        //save data into these static variables
        // the FIFO buffer is constantly provided with the data
        // the sampling is continuous adc0.emux.write(|w| w.em0().always()); //continuously sample, p1093
        // it means that the data is provided continuously but it has to be read out immediately as the interrupt is active
        //otherwise the new data will be discarded and oferflow will occur. It is impossible to recover after overflow, this is why 
        //we use panic error handling and the programme is stopped
        // the print out of data is given in SysTick() interrupt routine
        let ic_sample  = IC_SAMPLE.borrow(cs);
        let fbi_sample = FBI_SAMPLE.borrow(cs);
        let fv_sample  = FV_SAMPLE.borrow(cs);
        let fd_sample  = FD_SAMPLE.borrow(cs);
        let av_sample  = AV_SAMPLE.borrow(cs);
        let fbv_sample = FBV_SAMPLE.borrow(cs);
        
        ic_sample.set(adc0.ssfifo0.read().data().bits());
        fbi_sample.set(adc0.ssfifo0.read().data().bits());
        fv_sample.set(adc0.ssfifo0.read().data().bits()); 
        fd_sample.set(adc0.ssfifo0.read().data().bits());
        av_sample.set(adc0.ssfifo0.read().data().bits());
        fbv_sample.set(adc0.ssfifo0.read().data().bits());
        
        //Toggle LED on every entrance in the interrupt routine
        let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };            
        gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x02));        
    });
}
