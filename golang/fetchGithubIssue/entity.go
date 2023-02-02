package fetchGithubIssue

import "time"

const IssuesURL = "https://api.github.com/search/issues"

type User struct {
	Login   string
	HTMLURL string `json:"html_url"`
}

type Issue struct {
	Number     int
	HTMLURL    string `json:"html_url"`
	Title      string
	State      string
	User       *User
	CreactedAt time.Time `json:"creacted_at"`
	Body       string
}

type IssuesSearchResult struct {
	TotalCount int `json:"total_count"`
	Items      []*Issue
}
