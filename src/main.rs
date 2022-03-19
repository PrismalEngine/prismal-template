use prismal::prelude::*;

struct {{app_code_name}}App {
    // This is required to implement `AppCore`
    utils: Box<AppUtilities>,
}

impl AppCore for {{app_code_name}}App {
    fn info(&self) -> AppInfo {
        AppInfo {
            // Application name to show to the user
            name: "{{app_display_name}}".into(),            

            // Publisher name to show to the user 
            publisher: "{{publisher_display_name}}".into(),

            // Application version
            version: Version(0, 1, 0, 0),                   
        }
    }

    fn utils(&mut self) -> &mut AppUtilities {
        // Get and return a mutable reference to the `AppUtilities`
        self.utils.as_mut()
    }

    fn utils_ref(&self) -> &AppUtilities {
        // Get and return a reference to the `AppUtilities`
        self.utils.as_ref()
    }
}

impl AppEvents for {{app_code_name}}App {
    fn start(&mut self) -> Result<(), AppError> {
        // TODO: Initialize the application
        Ok(())
    }

    fn tick(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    fn render(&mut self) -> Result<(), AppError> {
        Ok(())
    }
}

impl AppFactory for {{app_code_name}}App {
    fn make_app() -> Box<Self> {
        // Construct an instance of `{{app_code_name}}App` on the heap
        Box::new(Self {
            utils: AppUtilities::new(),

            // Any fields you add to `{{app_code_name}}App` 
            // MUST be initialied here!
        })
    }
}

impl AppECS for {{app_code_name}}App {
    fn extra_initializers() -> Vec<Box<dyn ECSInitializer>> {
        // TODO: Add custom `ECSInitializer` structs to the `Vec` below
        vec![]
    }
}

fn main() {
    // Create/initialize an instance of `{{app_code_name}}App` and then run it.
    // The `entry` function never returns!
    entry::<{{app_code_name}}App>();

    // Anything after `entry` will not get executed!
}
