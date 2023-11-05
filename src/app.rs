use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::popup::PopupPage;
use crate::header::Header;
use crate::search_bar::SearchBar;
use crate::listing_prev::ListingPrev;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
 
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/popup-test" view=PopupPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    view! {
		<div class="home-page">
			<Header/>
			<SearchBar/>
			<div class="listings">
				<ListingPrev company_name="Company 1".to_string() position="Position 1".to_string() num_comments="1".to_string()/>
				<ListingPrev company_name="Company 2".to_string() position="Position 2".to_string() num_comments="2".to_string()/>
				<ListingPrev company_name="Company 3".to_string() position="Position 3".to_string() num_comments="3".to_string()/>
			</div>
		</div>
    }
}

/// Renders the new post page of your application.
/// This is where users can create new posts.
#[component]
fn NewPost() -> impl IntoView {
	view! {
		<h1>"New Post"</h1>
	}
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
