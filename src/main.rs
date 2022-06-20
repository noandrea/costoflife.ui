use costoflife::TxRecord;
use gloo_console as console;

use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::KeyboardEvent, html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    InputValue(String),
}

pub struct App {
    input: String,
    tx: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::from("1002€ 20y"),
            tx: String::default(),
        }
    }

    // update the screen data
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputValue(v) => {
                console::log!(&v);
                self.input = v.clone();

                match TxRecord::from_str(&v) {
                        Ok(tx) => self.tx = {
                            format!("It will cost you {}€ for {} days, that is until {}. Current progress is {:.0}%", tx.per_diem(), tx.get_duration_days(), tx.get_ends_on(), 100_f32*tx.get_progress(None))
                        },
                        Err(_) => self.tx = "I didn't get that!".to_string(),
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                let target: Option<EventTarget> = event.target();
                let inp = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                inp.map(|input| Msg::InputValue(input.value()))
            } else {
                None
            }
        });

        html! {

            <div class="grid grid-cols-1 gap-2">
                <div class="rounded-lg px-12 py-12 text-center bg-col-red  mb-12">
                    <h2 class="text-7xl mb-4 tracking-wider font-serif">
                        {"COST OF LIFE"}
                    </h2>
                    <p class="mb-2 mt-4">
                        {"CostOf.Life is an experiment in personal finance management loosely based
                        on the "}
                        <a
                            href="https://www.econlib.org/buying-boots"
                            class="underline"
                            target="_blank">
                            {"boots theory"}
                        </a>
                    </p>
                    <p class="mb-2">
                        {"The idea is simple, whenever you have an expense that is constant or
                        recurring or when you make a purchase for something that has a substantial
                        lifespan, you can split the cost of it over its lifetime and calculate a "}
                        <span class="italic font-semibold">{"per diem"}</span>
                        {"."}
                    </p>
                    <p class="mb-2">
                        {"The sum of all"}
                        <span class="italic">{"per diem"}</span>
                        {"gives your CostOf.Life baseline. That gives you an idea about how much you
                        need to earn to make your life costs sustainable and can help you decide
                        whenever an expense is worth making."}
                    </p>
                </div>
                <div class="rounded-lg px-12 py-12 bg-col-red mb-12 text-center">
                    <p>
                    {"This is the CostOf.Life calculator"}
                    </p>
                    <div class="my-4">
                        <input type="text" class="w-full bg-gray-100 rounded border border-gray-300
                        focus:border-col-blue text-base outline-none text-gray-700 py-1 px-3
                        leading-8 transition-colors duration-200 ease-in-out typing my-2
                        text-center" value={self.input.clone()} {onkeypress} data-examples='"Rent 923€ 1m12x #rent 01012021","Running shoes #clothing 69€ 2y","Modern Steel Flash #bike #transport 1900€ 10y 210320","Mobile data 9.99€ 28d12x","Netflix 7.99€ 010121 1m12x #entertainment 100320"'/>
                        <button
                            class="mx-auto p-2 text-center text-white transition bg-col-blue
                            rounded-md shadow ripple hover:shadow-lg hover:bg-costoflife-light-blue
                            focus:outline-none"
                        >
                            <svg
                            class="w-10"
                            version="1.1"
                            id="Layer_1"
                            xmlns="http://www.w3.org/2000/svg"
                            x="0px"
                            y="0px"
                            viewBox="0 0 291.764 291.764"
                            style="enable-background:new 0 0 291.764 291.764;"
                            >
                            <g>
                                <path
                                style="fill:#EAEDED;"
                                d="M36.47,0h218.824c10.066,0,18.235,8.169,18.235,18.235v255.294c0,10.066-8.169,18.235-18.235,18.235
                                H36.47c-10.066,0-18.235-8.169-18.235-18.235V18.235C18.234,8.16,26.404,0,36.47,0z" />
                                <path
                                style="fill:#D3D6D8;"
                                d="M45.587,27.353h200.588v63.824H45.587V27.353z" />
                                <path
                                style="fill:#B7BBBD;"
                                d="M191.47,45.588v27.353h36.471V45.588H191.47z
                                M218.823,63.824h-18.235v-9.118h18.235V63.824z" />
                                <path
                                style="fill:#49616E;"
                                d="M45.587,154.991h36.471V118.52H45.587V154.991z
                                M100.293,154.991h36.471V118.52h-36.471V154.991z
                                M154.999,118.529V155h36.471v-36.471H154.999z
                                M45.587,209.697h36.471v-36.461H45.587V209.697z
                                M100.293,209.697h36.471v-36.461 h-36.471V209.697z
                                M154.999,209.697h36.471v-36.461h-36.471V209.697z
                                M45.587,264.403h36.471v-36.471H45.587V264.403z
                                M100.293,264.403h36.471v-36.471h-36.471V264.403z
                                M154.999,264.403h36.471v-36.471h-36.471V264.403z" />
                                <path
                                style="fill:#E2574C;"
                                d="M209.705,118.529V155h36.471v-36.471H209.705z
                                M209.705,264.403h36.471v-91.167h-36.471V264.403z" />
                            </g>
                            </svg>
                        </button>
                    </div>
                    // Display the current value of the counter
                    <p class="counter">
                        {&self.tx}
                    </p>
                </div>
                <footer class="rounded-lg px-12 py-12 bg-col-blue">

                    <div class="container flex md:flex-row md:flex-nowrap flex-wrap flex-col">
                    <div class="flex-grow flex flex-wrap -mb-10 md:mt-0 mt-10 md:text-left
                        text-center">
                            <div class="lg:w-1/4 md:w-1/2 w-full px-4">
                                <h2 class="title-font font-medium text-col-pink tracking-widest text-sm mb-3">
                                {"Links"}
                                </h2>
                                <nav class="list-none mb-10">
                                    <li>
                                        <a
                                    class=" hover:text-white-800"
                                    href="https://dev.to/noandrea/rust-wasm-tailwind-svelte-2kgh">
                                    {"Tech article"}
                                    </a>
                                </li>
                            </nav>
                        </div>
                        <div class="lg:w-1/4 md:w-1/2 w-full px-4">
                            <h2
                            class="title-font font-medium text-col-pink tracking-widest text-sm
                            mb-3">
                            {"Sources"}
                            </h2>
                            <nav class="list-none mb-10">
                            <li>
                                <a
                                class="text-white-600 hover:text-white-800"
                                href="https://github.com/noandrea/costoflife-rs">
                                {"Library"}
                                </a>
                            </li>
                            <li>
                                <a
                                class="text-white-600 hover:text-white-800"
                                href="https://github.com/noandrea/costoflife.ui">
                                {"Ui"}
                                </a>
                            </li>
                            </nav>
                        </div>
                        <div class="lg:w-1/4 md:w-1/2 w-full px-4">
                            <h2
                            class="title-font font-medium text-col-pink tracking-widest text-sm
                            mb-3">
                            {"Other projects"}
                            </h2>
                            <nav class="list-none mb-10">
                            <li>
                                <a
                                class="text-white-600 hover:text-white-800"
                                href="https://meetvalis.com">
                                {"VALIS"}
                                </a>
                            </li>
                            </nav>
                        </div>
                    </div>
                </div>
                <div class="">
                    <div
                    class="container mx-auto py-4 flex flex-wrap flex-col sm:flex-row
                    text-center">
                    <p class="text-gray-500 text-sm sm:text-left">
                        {"© 2021 CostOf.Life —"}
                        <a
                        href="https://adgb.me"
                        rel="noopener noreferrer"
                        class="text-gray-600 ml-1"
                        target="_blank">
                        {"@adgb"}
                        </a>
                    </p>
                    <span
                        class="inline-flex sm:ml-auto sm:mt-0 mt-2 justify-center
                        sm:justify-start">
                        <a class="ml-3 text-gray-500" href="https://twitter.com/noandrea">
                        <svg
                            fill="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            class="w-5 h-5"
                            viewBox="0 0 24 24">
                            <path
                            d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66
                            10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5
                            4.5 0 00-.08-.83A7.72 7.72 0 0023 3z" />
                        </svg>
                        </a>
                        <a class="ml-3 text-gray-500" href="https://github.com/noandrea">
                        <svg
                            fill="currentColor"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24">
                            <path
                            d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207
                            11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729
                            1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304
                            3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931
                            0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0
                            1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005
                            2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653
                            1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0
                            4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0
                            .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386
                            0-6.627-5.373-12-12-12z" />
                        </svg>
                        </a>
                        <a
                        class="ml-3 text-gray-500"
                        href="https://linkedin.com/in/andreagiacobino/">
                        <svg
                            fill="currentColor"
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="0"
                            class="w-5 h-5"
                            viewBox="0 0 24 24">
                            <path
                            stroke="none"
                            d="M16 8a6 6 0 016 6v7h-4v-7a2 2 0 00-2-2 2 2 0 00-2 2v7h-4v-7a6 6
                            0 016-6zM2 9h4v12H2z" />
                            <circle cx="4" cy="4" r="2" stroke="none" />
                        </svg>
                        </a>
                    </span>
                    </div>
                </div>
                </footer>
            </div>

        }
    }
}

fn main() {
    yew::start_app::<App>();
}
