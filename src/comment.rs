use leptos::*;
use leptos_meta::*;
use leptos_router::*;


/*
HOW TO WRITE A COMMENT:

<Comment comment_data= Comment::new (
    String::from("Bob"),
    String::from("Hi. My name is bob. How is your day?"),
    26764,
    0.65
)/>
*/

// Struct for comment data
pub struct Comment {
    author: String,
    content: String,
    timestamp: u64,
    rating: f64,
}

// Implementation of getters for comment data
impl Comment {
    pub fn new(author: String, content: String, timestamp: u64, rating:f64) -> Self {
        Comment {
            author,
            content,
            timestamp,
            rating
        }
    }

    fn get_author(&self) -> &String {
        &self.author
    }

    fn get_content(&self) -> &String {
        &self.content
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_rating(&self) -> f64 {
        self.rating
    }
}

// Renders a navbar structure
#[component]
pub fn Comment(
    comment_data: Comment
) -> impl IntoView {
    // Progress data for star rating
    let progress = comment_data.get_rating();
    let filled_stars = (progress * 5.0).round() as usize;   
    // Generates stars based on the value of progress
    fn generate_star_icons(filled_stars: usize) -> Vec<impl IntoView> {
        (0..5).map(|i| {
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
    // Vector of SVGs
    let star_icons = generate_star_icons(filled_stars);

    view! {
        <div class="comment">
            <div class="comment-header"> 
                <div class="comment-profile">
                    {comment_data.get_author()}
                </div>
                <div class="star-rating">
                    <progress max="5" value={progress} class="hidden-progress"></progress>
                    <div class="stars">
                        {star_icons}
                    </div>
                </div>
                <div class ="comment-timestamp">
                {comment_data.get_timestamp()}
                </div>
            </div>
            <div class="comment-content">
                {comment_data.get_content()}
            </div>
        </div>
    }
}