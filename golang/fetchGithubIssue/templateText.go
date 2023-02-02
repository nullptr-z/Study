package fetchGithubIssue

import (
	"log"
	"os"
	"text/template"
	"time"
)

var templs = `{{.TotalCount}} issue:
{{range .Items}}--------
Number:	{{.Number}}
User:		{{.User.Login}}
Title:	{{.Title | printf "%.64s"}}
Age:		{{.CreatedAt | daysAgo}} days
{{end}}`

func daysAgo(t time.Time) int {
	return int(time.Since(t).Hours() / 24)
}

var reports = template.Must(template.New("issuelist").
	Funcs(template.FuncMap{"daysAgo": daysAgo}).
	Parse(templs))

func TemplateText() {
	param := []string{"repo:golang/go is:open json decoder"}
	result, err := SearchIssues(param)
	if err != nil {
		log.Fatal(err)
	}

	if err := reports.Execute(os.Stdout, result); err != nil {
		log.Fatal(err)
	}
}
