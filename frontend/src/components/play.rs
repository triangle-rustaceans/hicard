use yew::prelude::*;

struct Play {
    properties: (),
    link: ComponentLink<Self>,
}

impl Component for Play {
    type Properties = ();

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Play {
        Play { properties, link }
    }
}
