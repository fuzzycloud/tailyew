use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let table = html! {

      <div class="overflow-x-auto">
            <Table class="w-full">
            <thead>
            <tr>
              <th></th>
              <th>{"Name"}</th>
              <th>{"Job"}</th>
              <th>{"Favorite Color"}</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th>{"1"}</th>
              <td>{"Cy Ganderton"}</td>
              <td>{"Quality Control Specialist"}</td>
              <td>{"Blue"}</td>
            </tr>
            <tr>
              <th>{"2"}</th>
              <td>{"Hart Hagerty"}</td>
              <td>{"Desktop Support Technician"}</td>
              <td>{"Purple"}</td>
            </tr>
            <tr>
              <th>{"3"}</th>
              <td>{"Brice Swyre"}</td>
              <td>{"Tax Accountant"}</td>
              <td>{"Red"}</td>
            </tr>
          </tbody>
            </Table>
        </div>
    };

    html! {
        <div>
            <Display title="Table" preview={table} />
        </div>
    }
}
