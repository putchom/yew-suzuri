use nachiguro::AppBar;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;

pub struct Header {}

impl Component for Header {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    true
  }

  fn view(&self) -> Html {
    type Anchor = RouterAnchor<Route>;

    html! {
      <div class="Header">
        <AppBar>
          <Anchor
            classes="Header-logo"
            route=Route::HomePage
          >
            <svg class="Header-logo-image" enable-background="new 0 0 94 30.4" viewBox="0 0 94 30.4" xmlns="http://www.w3.org/2000/svg">
              <g fill="#000000">
                <path d="m59.2 11.6c0 1.4-1.2 2-2.4 2s-2.5-.7-2.5-2v-10.9c-1.3 0-2.6 0-3.9 0v11.1c0 3.5 3.1 5.6 6.4 5.6s6.4-2.1 6.4-5.6v-11.1c-1.3 0-2.7 0-4 0z"></path>
                <path d="m7.1 7.1c-1.5-.2-2.7-.4-2.6-1.6.1-2.1 4.5-2.2 4.5 0h4c.1-7.1-12.6-7.1-12.5 0 0 3.5 2.5 4.7 5.9 5 1.6.2 3 .4 3 1.5 0 2-5.1 1.9-5.1-.2-1.4 0-2.6 0-4 0-.1 7.2 13.2 7.1 13.2.2 0-4-3.2-4.6-6.4-4.9z"></path>
                <path d="m25.8 11.6c0 1.4-1.2 2-2.4 2s-2.5-.7-2.5-2v-10.9c-1.3 0-2.6 0-3.9 0v11.1c0 3.5 3.1 5.6 6.4 5.6s6.4-2.1 6.4-5.6v-11.1c-1.3 0-2.7 0-4 0z"></path>
                <path d="m46.3.7h-12.2v3.7h6l-6.6 12.4h12.9c0-1.4 0-2.3 0-3.7h-6.7z"></path>
                <path d="m74.6.7c-2.3 0-4.7 0-7.1 0v16.1h4.1v-4.7h2.1l2.8 4.7h4.9l-3.4-5.7c4.1-2.8 2.9-10.4-3.4-10.4zm0 7.7h-2.9c0-1.3 0-2.7 0-4h2.9c2.3 0 2.2 4 0 4z"></path>
                <path d="m93 4.3c0-1.4 0-3.6 0-3.6h-8v3.6h2v8.9h-2v3.6h8c0-1.4 0-2.3 0-3.6h-2v-8.9z"></path>
              </g>
              <path d="m33.85 28.14h.44c.02 0 .05-.01.06-.03l1.8-2.95.98 3.37c.01.02.03.04.05.04h1.89c.02 0 .03-.02.03-.04l-1.99-6.31c-.01-.02-.03-.04-.05-.04h-.71c-.02 0-.05.02-.06.03l-2.43 3.76-2.43-3.76c-.01-.02-.04-.03-.06-.03h-.71c-.02 0-.05.02-.05.04l-1.99 6.31c-.01.02.01.04.03.04h1.89c.02 0 .04-.02.05-.04l.98-3.37 1.8 2.95c.01.02.04.03.06.03z" fill="#005bab"></path>
              <path d="m28.69 25c0-.02-.02-.03-.04-.03h-.25-1.87-3.26c-.28 0-.52.23-.52.52 0 .28.23.52.52.52h3.2c-.38 1.01-1.67 1.74-3.2 1.74-1.83 0-3.31-1.06-3.31-2.36 0-1.31 1.48-2.37 3.31-2.37 1 0 1.89.32 2.5.82 0 0 0 0 .01.01h.02 2.2c.02 0 .04-.02.04-.04 0-.01 0-.02-.01-.02-.94-1-2.71-1.67-4.76-1.67-3.01 0-5.46 1.47-5.46 3.28s2.44 3.28 5.46 3.28c3.01 0 5.46-1.47 5.46-3.28 0-.15-.01-.28-.04-.4" fill="#005bab"></path>
              <path d="m44.42 22.1c-3.01 0-5.46 1.47-5.46 3.28s2.44 3.28 5.46 3.28c3.01 0 5.46-1.47 5.46-3.28s-2.44-3.28-5.46-3.28m0 5.64c-1.83 0-3.31-1.06-3.31-2.36 0-1.31 1.48-2.37 3.31-2.37s3.31 1.06 3.31 2.37c.01 1.3-1.48 2.36-3.31 2.36" fill="#005bab"></path>
              <path d="m72.88 28.47c0 .06.04.1.09.1h1.14c.06 0 .11-.05.11-.1l.2-2.61c0-.06-.04-.1-.09-.1h-1.14c-.06 0-.11.05-.11.1z" fill="#38393c"></path>
              <path d="m79.47 25.75c-.06 0-.1.05-.09.1l.2 2.61c0 .06.05.1.11.1h1.14c.06 0 .16-.05.16-.1l-.26-2.61c0-.06-.05-.1-.11-.1z" fill="#38393c"></path>
              <path d="m80.69 24.46c.06 0 .1-.05.1-.1v-1.1c0-.06-.05-.1-.1-.1h-3.05c-.06 0-.1-.05-.1-.1v-.77c0-.06-.05-.1-.1-.1h-1.1c-.06 0-.1.05-.1.1v.77c0 .06-.05.1-.1.1h-3.03c-.06 0-.1.05-.1.1v1.1c0 .06.05.1.1.1h3.03c.06 0 .1.05.1.1v3.9c0 .06.05.1.1.1h1.1c.06 0 .1-.05.1-.1v-3.9c0-.06.05-.1.1-.1z" fill="#38393c"></path>
              <path d="m81.42 23.7c0 .06.05.1.1.1h.77c.06 0 .1-.05.1-.1v-1.5c0-.06-.05-.1-.1-.1h-.77c-.06 0-.1.05-.1.1z" fill="#006ea8"></path>
              <path d="m82.92 23.7c0 .06.05.1.1.1h.77c.06 0 .1-.05.1-.1v-1.5c0-.06-.05-.1-.1-.1h-.77c-.06 0-.1.05-.1.1z" fill="#006ea8"></path>
              <path d="m51.57 26.65c-.03.03-.03.07 0 .1l.82.82c.03.03.07.03.1 0l2.98-2.98c.03-.03.07-.03.1 0l3.89 3.9c.03.03.07.03.1 0l.82-.82c.03-.03.03-.07 0-.1l-4.81-4.82c-.03-.03-.07-.03-.1 0z" fill="#38393c"></path>
              <path d="m59.55 24.02c.53 0 .96-.43.96-.96s-.43-.96-.96-.96-.96.43-.96.96.43.96.96.96" fill="#006ea8"></path>
              <path d="m62.55 28.5c0 .04.02.07.06.07h1.26c.04 0 .07-.03.08-.07l.68-5.68c0-.04-.02-.07-.06-.07h-1.26c-.04 0-.07.03-.08.07z" fill="#38393c"></path>
              <path d="m67.67 22.75c-.04 0-.06.03-.06.07l.69 5.68c0 .04.04.07.08.07h1.26c.04 0 .06-.03.06-.07l-.69-5.68c0-.04-.04-.07-.08-.07z" fill="#38393c"></path>
              <path d="m70.86 22.1c-.53 0-.96.43-.96.96s.43.96.96.96.96-.43.96-.96-.43-.96-.96-.96" fill="#006ea8"></path>
              <path d="m9.09 23.38v2.3h.02c.04-.05.09-.11.16-.17.06-.06.14-.11.23-.16s.19-.09.31-.13c.12-.03.25-.05.4-.05.23 0 .45.04.64.13.2.09.36.21.5.36s.25.34.33.55.12.43.12.67-.04.47-.11.68c-.08.21-.18.4-.32.55-.14.16-.31.28-.5.37-.2.09-.42.14-.67.14-.23 0-.45-.05-.65-.15s-.36-.25-.47-.44h-.03v.49h-.78v-5.15h.82zm1.88 3.51c0-.13-.02-.25-.06-.38-.04-.12-.1-.24-.17-.34-.08-.1-.18-.18-.29-.24-.12-.06-.26-.09-.42-.09-.15 0-.28.03-.4.1-.12.06-.22.15-.3.25s-.15.21-.19.34-.06.25-.06.38.02.25.06.38c.04.12.11.24.19.34s.19.18.3.24c.12.06.25.09.4.09.16 0 .3-.03.42-.1.12-.06.22-.15.29-.25.08-.1.13-.21.17-.34s.06-.26.06-.38" fill="#9a9da6"></path>
              <path d="m12.15 25.26h.91l.91 2.38h.01l.81-2.38h.85l-1.55 3.97c-.06.15-.12.28-.19.4s-.15.21-.25.29-.21.14-.34.18-.28.06-.47.06c-.07 0-.14 0-.21-.01s-.14-.02-.21-.04l.07-.71c.05.02.11.03.16.04s.1.01.15.01c.09 0 .16-.01.22-.03s.11-.05.15-.1c.04-.04.08-.1.11-.16s.07-.14.1-.23l.16-.42z" fill="#9a9da6"></path>
            </svg>
          </Anchor>
        </AppBar>
      </div>
    }
  }
}