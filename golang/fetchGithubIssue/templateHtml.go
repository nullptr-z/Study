package fetchGithubIssue

import (
	"html/template"
	"log"
	"os"
)

var templ = `
<H1>{{.TotalCount}} issues</H1>
<table>
<tr style='text-align: left'>
	<th>编号</th>
	<th>状态</th>
	<th>用户</th>
	<th>标题</th>
</tr>
{{range .Items}}
<tr>
	<td><a href='{{.HTMLURL}}'>{{.Number}}</a></td>
	<td>{{.State}}</td>
	<td><a href='{{.User.HTMLURL}}'>{{.User.Login}}</a></td>
	<td><a href='{{.HTMLURL}}'>{{.Title}}</a></td>
</tr>
{{end}}
</table>
`

var report = template.Must(template.New("report").Parse(templ))

func TemplateHtml() {
	param := []string{"repo:golang/go 3133 10535"}
	result, err := SearchIssues(param)
	if err != nil {
		log.Fatal(err)
	}

	if err := report.Execute(os.Stdout, result); err != nil {
		log.Fatal(err)
	}
}
