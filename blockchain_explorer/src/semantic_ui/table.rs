use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct Cell(pub String);

#[derive(Clone, Debug)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub handler: Option<Callback<()>>,
}

impl From<Vec<Cell>> for Row {
    fn from(cells: Vec<Cell>) -> Self {
        Row {
            cells,
            handler: None,
        }
    }
}
impl From<Vec<String>> for Row {
    fn from(v: Vec<String>) -> Self {
        Row {
            cells: v.into_iter().map(|v| Cell(v)).collect(),
            handler: None,
        }
    }
}

impl From<String> for Cell {
    fn from(v: String) -> Self {
        Cell(v)
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    header: Option<Row>,
    rows: Vec<Row>,
}

#[derive(Debug, Properties)]
pub struct TableProperties {
    pub header: Option<Row>,
    pub rows: Vec<Row>,
}

pub enum TableMessage {
    Click(usize),
}

impl Component for Table {
    type Message = TableMessage;
    type Properties = TableProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Table {
            header: props.header,
            rows: props.rows,
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.header = props.header;
        self.rows = props.rows;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TableMessage::Click(i) => {
                if let Some(ref c) = self.rows[i as usize].handler {
                    c.emit(())
                }
            }
        }
        true
    }
}

impl Renderable<Table> for Table {
    fn view(&self) -> Html<Self> {
        html! {
            <table class="ui celled table">
                {self.view_header()}
                { for self.rows.iter().enumerate().map(Self::view_row) }
            </table>
        }
    }
}

impl Table {
    fn view_header(&self) -> Html<Self> {
        match &self.header {
            Some(row) => html! {
            <tr>
                {
                for row.cells.iter().enumerate().map(|(j,cell)|
                html!{
                    <th>
                        {&*cell.0}
                    </th>
                })
                }
            </tr>

            },
            None => html! {},
        }
    }
    fn view_cell(cell: &Cell) -> Html<Self> {
        html! {
            <td>
                {&*cell.0}
            </td>
        }
    }

    fn view_row((i, row): (usize, &Row)) -> Html<Self> {
        html! {
            <tr onclick=|_| TableMessage::Click(i) >
                {
                for row.cells.iter().map(Self::view_cell)
                }
            </tr>
        }
    }
}
