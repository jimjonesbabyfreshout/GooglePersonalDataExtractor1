use yew::prelude::*;

struct Model {
  data: String,
}

enum Msg {
  Decode,
  Copy,
  Save,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self {
      data: String::new(),
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Decode => {
        // Decode the data from the HTML file.

        true
      }
      Msg::Copy => {
        // Copy the data to the clipboard.

        true
      }
      Msg::Save => {
        // Save the data to the device.

        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <>
        <h1>{ "Google Personal Data Extractor" }</h1>
        <p>{ "This tool will extract all of your Google personal data from a HTML file. Simply select the file you want to decode, and then click the \"Decode\" button. The extracted data will be displayed in the text viewer below." }</p>
        <form id="form">
          <input type="file" id="file-input" accept=".html" />
          <button type="submit" id="decode-button" onclick={ctx.link().callback(|_| Msg::Decode)}>{"Decode"}</button>
        </form>
        <div id="text-viewer"></div>
        <div id="buttons">
          <button id="copy-button" onclick={ctx.link().callback(|_| Msg::Copy)}>{"Copy to clipboard"}</button>
          <button id="save-button" onclick={ctx.link().callback(|_| Msg::Save)}>{"Save to device"}</button>
        </div>
      </>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}

