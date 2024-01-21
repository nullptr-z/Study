## What is template?

一些预先定义好的 HTML 页面，用于复用生成 HTML 文件

package (text/template, html/template), go 内置的模版引擎

- 模板引擎
- Action
- 参数、变量、管道
- 函数
- 模板组合

实用函数
tp.Lookup 在模版集里查找模板

## 解析/加载

- ParseFiles 解析文件,可以多个文件；返回第一个，其他的作为加入模版集 map,以后使用
- ParseGlob 按模式匹配文件来解析
- Parse 使用字符串模板

```go
func tempParse(w http.ResponseWriter, r *http.Request) {
  tp, _ := template.ParseFiles("template/temp.html")
  tp.Execute(w, "zhou")
  // 等效实现
  tp1 := template.New("template/temp.html")
  tp1, _ = tp1.ParseFiles("template/temp.html")
  // 匹配模式
  tp, _ = template.ParseGlob("template/temp*")
  // tp.Parse("重定义 {{ . }}")
  tp.Execute(w, "zhou")
}
```

## 执行模版

- Execute 适合用于单模板
- ExecuteTemplate 适合用于多模板
-

```go
func tempParses(w http.ResponseWriter, r *http.Request) {
  tp, _ := template.ParseFiles("template/temp.html", "template/temp2.html")
  // tp, _ = template.ParseGlob("template/temp*")
  tp.ExecuteTemplate(w, "my_template", "zhou")
}
```

## 模版生成 Demo

```go
func templateGenerate() {
  tempFiles := loadTemplates()
  http.Handle("/css/", http.FileServer(http.Dir("templates"))) // 加载css文件
  http.Handle("/", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
    fileName := r.URL.Path[1:] // 移除反斜杠
    tp := tempFiles.Lookup(fileName)
    if tp != nil {
      err := tp.Execute(w, "zz")
      if err != err {
        log.Fatalln(err.Error()) // 日志中记录错误
      }
      return
    }
    w.WriteHeader(http.StatusNotFound)
  }))
}

func loadTemplates() *template.Template {
  templateList := template.New("")
  // 如果加载出错Must会让程序Crash
  template.Must(templateList.ParseGlob("templates/*.html"))
  return templateList
}
```

## Action

```sh
<body>
  # if else
  {{ if . }}
  {{ else }}
  {{ end }}

  # range
  <ul>
    {{ range . }}
      <!-- 内层 . 表示Item -->
      <li>{{ . }}</li>
    {{ end }}
  </ul>

  {{ range $key,$value :=. }}

  # with
  改变前              {{ . }}
  使用with改变.值     {{ with "change value" }}
  在这期间 . 被了值   {{ . }}
  改变的作用于结束    {{ end }}
  恢复了原来的值      {{ . }}

  # 另一种用法
  {{ with "" }}
     no null {{ . }}
  {{ else }}
     null {{ .}}
  {{ end }}

  # 嵌套模版
  {{ template "template2" . }} # 后面这个 .是传给 template2的参数

  # pipline; 123被传给printf作为第二个参数
  {{ 123 | printf "%.2f" }}
</body>
```

## 自定义函数

```go
  funcMap := template.FuncMap{"name": func(){}}
```

## Layout

```go
{{ template "layout.html" .}}
```
