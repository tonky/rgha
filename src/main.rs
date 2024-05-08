use serde::Serialize;

fn main() {
    let wf = Workflow {
        name: "test-workflow".to_string(),
        trigger: On {
            push: EventPush::Branches(vec!["master".to_string()]),
            label: Some(Label {
                types: vec![LabelType::Created],
            }),
            page_build: Some(true),
        },
        jobs: vec![Job {
            name: "test".to_string(),
        }],
    };

    let yaml = serde_yml::to_string(&wf).unwrap();
    println!("{}", yaml);
}

#[derive(Serialize, PartialEq, Debug)]
struct Label {
    types: Vec<LabelType>,
}

#[derive(Serialize, PartialEq, Debug)]
enum EventPush {
    #[serde(rename = "branches")]
    Branches(Vec<String>),
    #[serde(rename = "tags")]
    Tags(Vec<String>),
}

#[derive(Serialize, PartialEq, Debug)]
enum Trigger {
    Single(Event),
    Multiple(Events),
    Custom(On),
}

#[derive(Serialize, PartialEq, Debug)]
struct On {
    #[serde(with = "serde_yml::with::singleton_map_recursive")]
    push: EventPush,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_build: Option<bool>,
}

#[derive(Serialize, PartialEq, Debug)]
struct Workflow {
    name: String,
    #[serde(rename = "on")]
    trigger: On,
    jobs: Vec<Job>,
}

type Events = Vec<Event>;

#[derive(Serialize, PartialEq, Debug)]
struct Job {
    name: String,
}

#[derive(Serialize, PartialEq, Debug)]
// issue_comment event has the created, edited, and deleted activity types
enum Event {
    Issue(Issue),
    IssueComment(IssueCommentEvent),
    Label(LabelType),
    #[serde(rename = "push")]
    Push(Vec<String>),
    PullRequest(BranchFilters),
    Fork,
}

#[derive(Serialize, PartialEq, Debug)]
enum LabelType {
    #[serde(rename = "created")]
    Created,
    Edited,
    Added,
    Removed,
}

#[derive(Serialize, PartialEq, Debug)]
enum IssueCommentEvent {
    Created,
    Edited,
    Deleted,
}

#[derive(Serialize, PartialEq, Debug)]
enum Issue {
    Opened,
    Labeled,
}

type BranchFilters = Vec<String>;

impl Workflow {
    fn shell_commands(&self) -> String {
        "ok".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_workflow() {
        /*
          // build actions library from provided
          // assemble workflow from them
          given workflow
            - 'Workflow' instance
          conditions
            - create PR
          expected actions
            - lint, test, build, release
        */
        /*
                let wf = Workflow {
                    name: "test-workflow".to_string(),
                    on: On {
                        push: vec![Event::Push(vec!["master".to_string()])],
                        label: None,
                        page_build: None,
                    },
                    jobs: vec![
                        Job {
                            name: "test".to_string(),
                        },
                        Job {
                            name: "release".to_string(),
                        },
                    ],
                };

                let want = r#"name: "test-workflow"
        on:
          push:
            branches:
              - master
        jobs:
          - test
          - release
        "#;

                assert_eq!(wf.shell_commands(), want, "shell_commands failed");
         */
    }
}
