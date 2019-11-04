use failure::Error;
use stegos_blockchain::api::MacroBlockInfo;
use yew::format::Json;
use yew::prelude::*;

pub struct MacroBlockModel {
    //    props: MacroBlockProperties,
}

#[derive(Debug, Properties)]
pub struct MacroBlockProperties {
    #[props(required)]
    block: MacroBlockInfo,
}

impl Component for MacroBlockModel {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        MacroBlockModel {}
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        //        self.props = props;
        //        list.request_block_count();
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<MacroBlockModel> for MacroBlockModel {
    fn view(&self) -> Html<Self> {
        html! {
        <div>
        {"test"}
        </div>
        }
    }
}
