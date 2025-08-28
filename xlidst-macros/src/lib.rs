use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let block = input_fn.block;

    let expanded = quote! {
        use nannou::{app::App};
        use xlidst::{start, Model, get_context};

        fn get_slideshow(ctx: Context) -> Slideshow {
            #block
        }

        fn model(app: &App) -> Model {
            let slideshow = get_slideshow(get_context(app));
            let slides = slideshow.to_view_slides(app);
            return Model { current_slide: 0, slides };
        }

        fn main() {
            start(model);
        }
    };

    expanded.into()
}
