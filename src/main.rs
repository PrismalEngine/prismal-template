use prismal::prelude::*;

struct {{app_code_name}}App {
    // This is required to implement `AppCore`
    utils: Box<AppUtilities>,
}

impl AppCore for {{app_code_name}}App {
    fn info(&self) -> AppInfo {
        AppInfo {
            name: "{{app_display_name}}".into(),
            publisher: "{{app_publisher_name}}".into(),
            version: Version(0, 1, 0, 0),
        }
    }

    fn utils(&mut self) -> &mut AppUtilities {
        self.utils.as_mut()
    }

    fn utils_ref(&self) -> &AppUtilities {
        self.utils.as_ref()
    }
}

impl AppEvents for {{app_code_name}}App {
    fn start(&mut self) -> Result<(), AppError> {
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
        Box::new(Self {
            utils: AppUtilities::new()
        })
    }
}

impl AppECS for {{app_code_name}}App {
    fn extra_initializers() -> Vec<Box<dyn ECSInitializer>> {
        // TODO: Add custom `ECSInitializer` structs to the `Vec` below!
        vec![]
    }
}

// The `main` function should always look like this!
fn main() {
    entry::<{{app_code_name}}App>();
    // Anything after `entry` will not get run!
}
