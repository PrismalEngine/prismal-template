use prismal::prelude::*;
struct {{app_name}} {
    utils: Box<AppUtilities>,
}

impl AppCore for {{app_name}} {
    fn info(&self) -> AppInfo {
        AppInfo {
            name: "Sample App".into(),
            publisher: "Sample Publisher".into(),
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

impl AppEvents for {{app_name}} {
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

impl AppFactory for {{app_name}} {
    fn make_app() -> Box<Self> {
        Box::new(Self {
            utils: AppUtilities::new()
        })
    }
}

impl AppECS for {{app_name}} {
    fn extra_initializers() -> Vec<Box<dyn ECSInitializer>> {
        vec![]
    }
}

fn main() {
    entry::<{{app_name}}>();
}
