use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

/// Defines `appContent` for [`MatDrawer`].
///
/// If the child passed is an element (a `VTag`), then it is modified to include the appropriate attributes.
/// Otherwise, the child is wrapped in a `span` containing said attributes.
pub struct MatDrawerAppContent {
    props: Props
}

impl Component for MatDrawerAppContent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let children = self.props.children.iter().map(|child| {
            match child {
                Html::VTag(mut vtag) => {
                    vtag.add_attribute("slot", "appContent");
                    Html::VTag(vtag)
                }
                _ => html! {
                    <span slot="appContent">
                        { child }
                    </span>
                }
            }
        }).collect::<Html>();

        html! {
            { children }
        }
    }
}