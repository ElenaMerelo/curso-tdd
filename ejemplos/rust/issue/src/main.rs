#[derive(Debug)]
enum IssueState {
    Open,
    Closed,
}

#[derive(Debug)]
struct Issue {
    state: IssueState,
    project_name: String,
    issue_id: u64,
}

fn issue_factory( project_name: String,
                  issue_id: u64) -> Issue {
    return Issue { state: IssueState::Closed,
                   project_name: project_name,
                   issue_id: issue_id }
}

fn main() {
    let this_issue = issue_factory(String::from("CoolProject"), 1 );
    println!("{:?}", this_issue);
    let mut that_issue = issue_factory( String::from("CoolProject"), this_issue.issue_id + 1 );
    that_issue.state = IssueState::Open; // Avoid warning
    println!("{:?}", that_issue);
}
