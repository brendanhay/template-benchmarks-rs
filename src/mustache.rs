use criterion;
use mustache::{Context, Template};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
struct Inner {
    inner: Vec<usize>,
}

#[derive(Serialize)]
struct Outer {
    outer: Vec<Inner>,
}

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut outer = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        outer.push(Inner { inner });
    }

    let tmpl = mustache::compile_str(BIG_TABLE_TEMPLATE).unwrap();
    let data = mustache::to_data(Outer { outer }).unwrap();

    let _ = tmpl.render_data_to_string(&data);
    b.iter(|| tmpl.render_data_to_string(&data));
}

static BIG_TABLE_TEMPLATE: &'static str = "<html>
  {{#outer}}
    <tr>{{#inner}}<td>{{.}}</td>{{/inner}}</tr>
  {{/outer}}
</html>";

#[derive(Serialize)]
struct Teams<'a> {
    year: u16,
    teams: Vec<Team<'a>>,
}

#[derive(Serialize)]
struct Team<'a> {
    name: &'a str,
    score: u8,
    champ: bool,
}

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
    let tmpl = mustache::compile_str(TEAMS_TEMPLATE).unwrap();
    let data = mustache::to_data(Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu",
                score: 43,
                champ: true,
            },
            Team {
                name: "Beijing",
                score: 27,
                champ: false,
            },
            Team {
                name: "Guangzhou",
                score: 22,
                champ: false,
            },
            Team {
                name: "Shandong",
                score: 12,
                champ: false,
            },
        ],
    })
    .unwrap();

    let _ = tmpl.render_data_to_string(&data);
    b.iter(|| tmpl.render_data_to_string(&data));
}

static TEAMS_TEMPLATE: &'static str = r#"<html>
  <head>
    <title>{{year}}</title>
  </head>
  <body>
    <h1>CSL {{year}}</h1>
    <ul>
    {{#teams}}
      <li class="{{#champ}}champion{{/champ}}">
      <b>{{name}}</b>: {{score}}
      </li>
    {{/teams}}
    </ul>
  </body>
</html>"#;
