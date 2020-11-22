#![no_main]
#![no_std]

use snakebit as _; // global logger + panicking-behavior + memory layout

use microbit::hal::lo_res_timer::{LoResTimer, FREQ_16HZ};
use microbit::hal::nrf51;

use microbit::display::{self, Display, Frame, MicrobitDisplayTimer, MicrobitFrame};

use rtic::app;

use snakebit::GameState;

#[app(device = microbit::hal::nrf51, peripherals = true)]
const APP: () = {
    struct Resources {
        state: GameState,
        gpio: nrf51::GPIO,
        gpiote: nrf51::GPIOTE,
        display_timer: MicrobitDisplayTimer<nrf51::TIMER1>,
        display: Display<MicrobitFrame>,
        game_timer: LoResTimer<nrf51::RTC0>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut p: nrf51::Peripherals = cx.device;

        defmt::info!("init");

        p.GPIO.pin_cnf[26].write(|w| {
            w.dir()
                .input()
                .drive()
                .s0s1()
                .pull()
                .disabled()
                .sense()
                .disabled()
                .input()
                .connect()
        });
        p.GPIO.pin_cnf[17].write(|w| {
            w.dir()
                .input()
                .drive()
                .s0s1()
                .pull()
                .disabled()
                .sense()
                .disabled()
                .input()
                .connect()
        });

        p.GPIOTE.config[0]
            .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
        p.GPIOTE.intenset.write(|w| w.in0().set_bit());
        p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

        p.GPIOTE.config[1]
            .write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
        p.GPIOTE.intenset.write(|w| w.in1().set_bit());
        p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

        let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
        display::initialise_display(&mut timer, &mut p.GPIO);

        // Starting the low-frequency clock (needed for RTC to work)
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.reset();

        let mut game_timer = LoResTimer::new(p.RTC0);
        // 16Hz; 62.5ms period
        game_timer.set_frequency(FREQ_16HZ);
        game_timer.enable_tick_event();
        game_timer.enable_tick_interrupt();
        game_timer.start();

        init::LateResources {
            state: GameState::new(),
            gpio: p.GPIO,
            gpiote: p.GPIOTE,
            display_timer: timer,
            display: Display::new(),
            game_timer,
        }
    }

    #[task(binds = GPIOTE, priority = 1, resources=[gpiote, display, state])]
    fn btn(cx: btn::Context) {
        let gpiote = cx.resources.gpiote;
        let a_pressed = gpiote.events_in[0].read().bits() != 0;
        let b_pressed = gpiote.events_in[1].read().bits() != 0;

        /* Clear events */
        gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
        gpiote.events_in[1].write(|w| unsafe { w.bits(0) });

        if a_pressed {
            cx.resources.state.turn_left();
        }
        if b_pressed {
            cx.resources.state.turn_right();
        }
    }

    #[task(binds = RTC0, priority = 1,
        resources = [game_timer, state, display])]
    fn game_tick(cx: game_tick::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        &cx.resources.game_timer.clear_tick_event();
        cx.resources.state.game_tick();
        FRAME.set(&snakebit::render(&cx.resources.state.snake));
        cx.resources.display.set_frame(&FRAME);
    }

    #[task(binds = TIMER1, priority = 1, resources = [display_timer, gpio, display])]
    fn timer1(mut cx: timer1::Context) {
        display::handle_display_event(
            &mut cx.resources.display,
            cx.resources.display_timer,
            cx.resources.gpio,
        );
    }
};
