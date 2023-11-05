use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::comment::Comment;

// Listing contains information for a company's internship listing along with a list of comments
#[derive(Clone)]
pub struct Listing {
    company: String,
    description: String,
    url: String,
}

// Implementation of getters for comment data
impl Listing {
    pub fn new(company: String, description: String, url: String) -> Self {
        Listing {
            company,
            description,
            url,
        }
    }

    pub fn get_company(&self) -> &String {
        &self.company
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }
}

// Renders a navbar structure
#[component]
pub fn Listing(
    listing_data: ReadSignal<Listing>
) -> impl IntoView {
    // Create initial list and store as a signal
    let (comments, set_comments) = create_signal(vec![
        Comment::new (
            String::from("Bob"),
            String::from("Hi. My name is bob. How is your day?"),
            26764,
            0.65,
            1
        ),
        Comment::new (
            String::from("John"),
            String::from("Hi. My name is John. I am a bot."),
            26764,
            0.8,
            2
        ),
        Comment::new (
            String::from("Jane"),
            String::from("I hate this job"),
            26764,
            0.2,
            3
        )
    ] as Vec<Comment>);
    // Average rating, precalled to the above list of sample comments
    let (avg_rating, set_avg_rating) = create_signal(get_avg_rating(&comments.get()));

    // Function to add a comment to a list. Updates the list of comments rendered and the average (the star rating)
    let add_comment = move |_| {
        let new_comment = Comment::new(
            String::from("Bob"),
            String::from("Hi. My name is bob. How is your day?"),
            26764,
            0.65,
            14
        );
        // Add the comment to the list of comments
        set_comments.update(move |comments| {
            comments.push(new_comment);
        });

        // Calculate and update the average rating
        let average = get_avg_rating(&comments.get());
        set_avg_rating(average);
    };
    
    // Gets an average rating given a list of comments
    fn get_avg_rating(comment_list: &Vec<Comment>) -> f64 {
        let mut total_rating = 0.0;
        let mut count = 0.0;
        for c in comment_list.iter() {
            count += 1.0;
            total_rating += c.get_rating();
        }

        total_rating/count
    }

    // Generates stars based on the value of progress
    fn generate_star_avg(filled_stars: usize) -> Vec<impl IntoView> {
        (1..6).map(|i| {
            if i < filled_stars {
                view! {
                    <svg class = "star" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-star-filled" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffbf00" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                    <path d="M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z" stroke-width="0" fill="currentColor" />
                </svg>
                }
            } else {
                view! {
                    <svg class = "star" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-star" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffbf00" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                    <path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z" />
                    </svg>
                }
            }
        }).collect()
    }

    view! {
        <div class="listing">
            <div class="listing-main">
                <div class="listing-header"> 
                    <div class="listing-company">
                        {listing_data.get().get_company()}
                    </div>
                    <div class="star-rating">
                        <div class="stars">
                            {generate_star_avg((avg_rating.get() * 5.0).round() as usize)}
                        </div>
                        <div class="star-count">
                            {avg_rating.get()*5.0}
                        </div>
                    </div>
                    <div class ="listing-url">
                    {listing_data.get().get_url()}
                    </div>
                </div>
                <div class ="listing-description">
                        {listing_data.get().get_description()}
                </div>
            </div>
            <div class="comment-container">
                <For
                each = comments
                key = |c| c.get_id()
                children=move |c: Comment| {
                    view! {
                        <Comment comment_data=c></Comment>
                    }
                }
                ></For>
            </div>

            <button on:click=add_comment>
                "Add Dummy Comment"
            </button>
        </div>
    }
}