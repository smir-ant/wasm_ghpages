// use leptos::logging::log;  // стандартные средства логирования leptos, опять-таки, вдруг пригодится
use leptos::*;

#[derive(Debug, Clone)]
pub struct CounterData {
    pub id: u32,
    pub value: (ReadSignal<i32>, WriteSignal<i32>),
}

impl CounterData {
    pub fn new(id: &mut u32, value: &mut i32) -> Self {
        let counter = Self {
            id: id.to_owned(),
            value: create_signal(value.clone()),
        };
        *id += 1;
        *value += 10;
        counter
    }
}

/// Функция, создающая компонент Counter
/// Параметры:
/// - `counter`: сигнал для отслеживания значения счетчика
/// - `on_increase_click`: опциональный колбэк для увеличения значения
/// - `on_decrease_click`: опциональный колбэк для уменьшения значения
/// - `on_remove_click`: опциональный колбэк для удаления счетчика
#[component]
pub fn Counter(
    counter: ReadSignal<i32>,
    #[prop(into, optional)] on_increase_click: Option<Callback<i32>>,
    #[prop(into, optional)] on_decrease_click: Option<Callback<()>>,
    #[prop(into, optional)] on_remove_click: Option<Callback<()>>,
) -> impl IntoView {
    // Замыкание для обработки клика по кнопке увеличения
    let increase = move |_| {
        on_increase_click.as_ref().map(|f| f(counter()));
    };
    // Замыкание для обработки клика по кнопке уменьшения
    let decrease = move |_| {
        on_decrease_click.as_ref().map(|f| f(()));
    };
    // Замыкание для обработки клика по кнопке удаления
    let remove = move |_| {
        on_remove_click.as_ref().map(|f| f(()));
    };

    // Определение структуры представления (view) для компонента
    view! {
        <div class="counter">
            <button on:click=increase>"Увеличить"</button>
            <button on:click=decrease>"Уменьшить"</button>
            <span class="counter_label">{counter}</span>
            <button class="text-red-400" on:click=remove>"Удалить"</button>
        </div>
    }
}

/// Домашняя страница
#[component]
pub fn Home() -> impl IntoView {
    // Инициализация счетчика идентификаторов
    let mut id = 0u32;
    // Инициализация начального значения счетчика
    let mut initial_value = 0i32;
    // Создание сигнала для списка счетчиков
    let (counters, set_counters) = create_signal::<Vec<CounterData>>(vec![
        CounterData::new(&mut id, &mut initial_value),
        CounterData::new(&mut id, &mut initial_value),
        CounterData::new(&mut id, &mut initial_value),
    ]);

    // Функция для добавления нового счетчика
    let mut add_counter = move || {
        set_counters
            .update(|counters| counters.push(CounterData::new(&mut id, &mut initial_value)));
    };

    // Обработчик клика для удаления счетчика
    let handle_click_remove = move |id: u32| {
        set_counters.update(|counters| {
            counters.retain(|c| match c.id != id {
                true => true,
                false => {
                    c.value.0.dispose();  // данный сигнал больше не нужен и может быть удалён
                    false
                }
            })
        })
    };

    // Обработчик клика для увеличения значения счетчика
    let handle_increase_click = move |set_count: WriteSignal<i32>| {
        set_count.update(|v| *v += 10);
    };

    // Обработчик клика для уменьшения значения счетчика
    let handle_decrease_click = move |set_count: WriteSignal<i32>| {
        set_count.update(|v| *v -= 10);
    };

    // Определение структуры представления (view) для домашней страницы
    view! {
        <button on:click=move|_| { add_counter(); }>"Добавить счётчик"</button>
        <For each=counters key=|counter| counter.id children=move |counter| view! {
            <Counter counter={counter.value.0}
                on_increase_click = move |_value| {
                    // log!("{_value}");
                    handle_increase_click(counter.value.1)
                }
                on_remove_click = move |_| {
                    handle_click_remove(counter.id)
                }
                on_decrease_click = move |_| {
                    handle_decrease_click(counter.value.1)
                }
            />
        } />
    }
}
