use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
struct Question {
    question: &'static str,
    options: Vec<&'static str>,
    answer: usize,
}

#[derive(Properties, PartialEq)]
struct WrapperProps {
    pub children: Children,
}

#[function_component(Wrapper)]
fn wrapper(props: &WrapperProps) -> Html {
    html! {
        <div style="padding: 20px; border: 2px solid #333; width: 60%; margin: auto;">
            { for props.children.iter() }
        </div>
    }
}

#[function_component(Quiz)]
fn quiz() -> Html {
    let questions = vec![
        Question {
            question: "What is the capital of France?",
            options: vec!["Berlin", "Madrid", "Paris", "Lisbon"],
            answer: 2,
        },
        Question {
            question: "Which is the largest planet?",
            options: vec!["Earth", "Jupiter", "Mars", "Venus"],
            answer: 1,
        },
        Question {
            question: "Who wrote '1984'?",
            options: vec!["Orwell", "Shakespeare", "Dickens", "Austen"],
            answer: 0,
        },
    ];

    let current = use_state(|| 0);
    let score = use_state(|| 0);
    let name = use_state(|| "".to_string());
    let input_ref = use_node_ref();

    let on_answer = {
        let current = current.clone();
        let score = score.clone();
        let questions = questions.clone();
        Callback::from(move |index: usize| {
            if index == questions[*current].answer {
                score.set(*score + 1);
            }
            if *current < questions.len() {
                current.set(*current + 1);
            }
        })
    };

    let on_restart = {
        let current = current.clone();
        let score = score.clone();
        let name = name.clone();
        Callback::from(move |_| {
            current.set(0);
            score.set(0);
            name.set("".to_string());
        })
    };

    let on_submit_name = {
        let name = name.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                name.set(input.value());
            }
        })
    };

    let final_score = use_memo(
        ((*name).clone(), *score),
        {
            let total = questions.len();
            move |(name, score)| {
                if name.is_empty() {
                    format!("Score: {}/{}", score, total)
                } else {
                    format!("{}'s Score: {}/{}", name, score, total)
                }
            }
        }
    );

    html! {
        <div style="text-align: center;">
            <h1>{ "Quiz App" }</h1>
            <p>{ &*final_score }</p>

            { if *current < questions.len() {
                let q = &questions[*current];

                html! {
                    <>
                        <h2>{ q.question }</h2>
                        <ul style="list-style: none; padding: 0;">
                            { for q.options.iter().enumerate().map(|(i, option)| html! {
                                <li key={i}>
                                    <button onclick={on_answer.reform(move |_| i)}>{ option }</button>
                                </li>
                            }) }
                        </ul>
                    </>
                }
            } else {
                html! {
                    <>
                        <h2>{ "Quiz Complete!" }</h2>
                        <input type="text" placeholder="Enter your name" ref={input_ref.clone()} />
                        <button onclick={on_submit_name}>{ "Submit Name" }</button>
                        <p></p>
                        <button onclick={on_restart}>{ "Restart" }</button>
                    </>
                }
            }}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Wrapper>
            <Quiz />
        </Wrapper>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
