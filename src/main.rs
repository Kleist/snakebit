#![no_main]
#![no_std]

use snakebit as _; // global logger + panicking-behavior + memory layout

use heapless::Vec;

use microbit::hal::nrf51;

use microbit::display::{self, Display, Frame, MicrobitDisplayTimer, MicrobitFrame};

use rtic::app;

use snakebit::Coord;
use snakebit::Direction;
use snakebit::GameState;

#[app(device = microbit::hal::nrf51, peripherals = true)]
const APP: () = {
    struct Resources {
        state: GameState,
        gpio: nrf51::GPIO,
        gpiote: nrf51::GPIOTE,
        display_timer: MicrobitDisplayTimer<nrf51::TIMER1>,
        display: Display<MicrobitFrame>,
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

        let mut state = GameState {
            snake: Vec::new(),
            dir: Direction::North,
        };
        let _ = state.snake.push(Coord { x: 2, y: 0 });

        let mut timer = MicrobitDisplayTimer::new(p.TIMER1);
        display::initialise_display(&mut timer, &mut p.GPIO);

        init::LateResources {
            state: state,
            gpio: p.GPIO,
            gpiote: p.GPIOTE,
            display_timer: timer,
            display: Display::new(),
        }
    }

    #[task(binds = GPIOTE, priority = 1, resources=[gpiote, display, state])]
    fn btn(mut cx: btn::Context) {
        static mut FRAME: MicrobitFrame = MicrobitFrame::const_default();
        let gpiote = cx.resources.gpiote;
        let a_pressed = gpiote.events_in[0].read().bits() != 0;
        let b_pressed = gpiote.events_in[1].read().bits() != 0;
        let _ = defmt::info!(
            "Button pressed {:?}\n\r",
            match (a_pressed, b_pressed) {
                (false, false) => "",
                (true, false) => "A",
                (false, true) => "B",
                (true, true) => "A + B",
            }
        );

        /* Clear events */
        gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
        gpiote.events_in[1].write(|w| unsafe { w.bits(0) });

        let state = &mut cx.resources.state;
        if a_pressed {
            snakebit::turn_left(state);
        }
        if b_pressed {
            snakebit::turn_right(state);
        }
        if snakebit::step(state) {
            FRAME.set(&snakebit::render(&state.snake));
            cx.resources.display.lock(|display| {
                display.set_frame(&FRAME);
            });
        }
        else {
            defmt::info!("Game over");
        }
    }

    #[task(binds = TIMER1, priority = 2, resources = [display_timer, gpio, display])]
    fn timer1(mut cx: timer1::Context) {
        display::handle_display_event(
            &mut cx.resources.display,
            cx.resources.display_timer,
            cx.resources.gpio,
        );
    }
};
