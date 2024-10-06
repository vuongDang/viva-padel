mod availaibility_calendar;
mod day_availability;

use leptos::*;
use thaw::*;
use thaw::mobile::*;
use crate::book_court::availaibility_calendar::AvailaibilityCalendar;

#[component]
pub fn BookCourtView() -> impl IntoView {
    let value = create_rw_signal(String::from("calendar"));

    view! {
        <Tabs value>
            <Tab key="filter" label="Filter">
                "Filter"
            </Tab>
            <Tab key="calendar" label="Calendar">
                <AvailaibilityCalendar />
            </Tab>
            <Tab key="next_courts" label="Next Courts">
                "Next courts"
            </Tab>
        </Tabs>
//         <div style="height: 100vh;">
//             {move || value.get()}
//             <Tabbar value>
//             <TabbarItem key="filter">"Filter"</TabbarItem>
//             <TabbarItem key="calendar" children=AvailaibilityCalendar
// >"Calendar"</TabbarItem>
//             <TabbarItem key="next_courts">
//             "Next Courts"
//             </TabbarItem>
//             </Tabbar>
//         </div>
    }
}

