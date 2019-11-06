#![allow(non_snake_case)]
use yew::prelude::*;

use core::fmt::Display;
/// Representation of cell array.
// Yew force 'static lifetime for component.
pub trait Cells: 'static {
    const COUNT: usize;
    fn get(&self, idx: usize) -> &dyn Display;
}

pub trait WithHeader: Cells {
    fn header() -> Vec<&'static dyn Display>;
}

macro_rules! count {
    ($x:expr) => (1usize);
    ( $x:expr, $($xs:expr),* ) => (1usize + count!($($xs),*));
}

macro_rules! impl_cell {
    ($(($types:ident => $num:expr)),* $(,)?) => {
        impl<$($types: Display + 'static),*> Cells for ($($types,)*) {
            const COUNT: usize = count!($($num),*);
            fn get(&self, idx: usize) -> &dyn Display {
                let ($(ref $types,)*) = *self;
                match idx {
                    $($num => $types as &dyn Display,)*
                    _ => unreachable!("Expected get with value lower than = {}", Self::COUNT)
                }
            }
        }
        peel!($(($types => $num)),*);
    };
}

macro_rules! peel {
    (($_types:ident => $_num:expr)) => ();
    (($_types:ident => $_num:expr), $(($types:ident => $num:expr)),+) => (impl_cell! { $(($types => $num)),* })
}

impl_cell! {
    (T10 => 10),
    (T9 => 9),
    (T8 => 8),
    (T7 => 7),
    (T6 => 6),
    (T5 => 5),
    (T4 => 4),
    (T3 => 3),
    (T2 => 2),
    (T1 => 1),
    (T0 => 0),
}

#[derive(Clone, Debug)]
pub struct Row<T> {
    pub cells: T,
    pub handler: Option<Callback<()>>,
}

#[derive(Clone, Debug)]
pub struct Table<T> {
    props: TableProperties<T>,
}

#[derive(Debug, Clone, Properties)]
pub struct TableProperties<T> {
    pub header: Option<Vec<String>>,
    pub rows: Vec<Row<T>>,
    pub striped: bool,
    pub celled: bool,
    pub pagination: bool,
}

impl<T> Default for TableProperties<T> {
    fn default() -> Self {
        TableProperties {
            header: None,
            rows: vec![],
            striped: false,
            celled: false,
            pagination: false,
        }
    }
}

impl<T: WithHeader> From<Vec<Row<T>>> for TableProperties<T> {
    fn from(rows: Vec<Row<T>>) -> Self {
        let mut header: Vec<String> = Default::default();
        for el in T::header().into_iter() {
            header.push(el.to_string());
        }
        TableProperties {
            header: header.into(),
            rows,
            striped: false,
            celled: false,
            pagination: false,
        }
    }
}

impl<T: WithHeader> From<Vec<T>> for TableProperties<T> {
    fn from(rows: Vec<T>) -> Self {
        let mut header: Vec<String> = Default::default();
        for el in T::header().into_iter() {
            header.push(el.to_string());
        }
        let rows = rows
            .into_iter()
            .map(|c| Row {
                cells: c,
                handler: None,
            })
            .collect();

        TableProperties {
            header: header.into(),
            rows,
            striped: false,
            celled: false,
            pagination: false,
        }
    }
}

pub enum TableMessage {
    Click(usize),
}

impl<T: Cells> Component for Table<T> {
    type Message = TableMessage;
    type Properties = TableProperties<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Table { props }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TableMessage::Click(i) => {
                if let Some(ref c) = self.props.rows[i as usize].handler {
                    c.emit(())
                }
            }
        }
        true
    }
}

impl<T: Cells> Renderable<Table<T>> for Table<T> {
    fn view(&self) -> Html<Self> {
        let mut class = "ui selectable celled striped table";
        if self.props.pagination {
            class = "ui tablesorter selectable celled striped table"
        }
        html! {
            <table class={class}>
                {self.view_header()}
                { for self.props.rows.iter().enumerate().map(Self::view_row) }
            </table>
        }
    }
}

impl<T: Cells> Table<T> {
    fn view_header(&self) -> Html<Self> {
        match &self.props.header {
            Some(row) => html! {
            <tr>
                {
                for row.iter().enumerate().map(|(j,cell)|
                html!{
                    <th>
                        {&*cell}
                    </th>
                })
                }
            </tr>

            },
            None => html! {},
        }
    }
    fn view_cell(row: &Row<T>, cell: usize) -> Html<Self> {
        html! {
            <td>
                {row.cells.get(cell)}
            </td>
        }
    }

    fn view_row((i, row): (usize, &Row<T>)) -> Html<Self> {
        html! {
            <tr onclick=|_| TableMessage::Click(i) >
                {
                    for (0..T::COUNT).map(|i| Self::view_cell(row, i))
                }
            </tr>
        }
    }
}
