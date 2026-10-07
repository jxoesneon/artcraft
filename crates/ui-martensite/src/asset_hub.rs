//! Project hub data model.

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectRecord {
    pub id: u64,
    pub title: String,
    pub app_type: String,
}

pub struct ProjectHub {
    pub projects: Vec<ProjectRecord>,
}

impl ProjectHub {
    pub fn new() -> Self {
        Self { projects: Vec::new() }
    }

    pub fn create_project(&mut self, title: &str, app_type: &str) -> u64 {
        let id = self.projects.len() as u64 + 1;
        self.projects.push(ProjectRecord {
            id,
            title: title.to_string(),
            app_type: app_type.to_string(),
        });
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hub_creation() {
        let mut hub = ProjectHub::new();
        let id = hub.create_project("SciFi Short", "filmcraft");
        assert_eq!(id, 1);
        assert_eq!(hub.projects.len(), 1);
    }
}
