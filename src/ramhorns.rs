use criterion;
use ramhorns::{Content, Template};

#[derive(Content)]
struct Outer {
    outer: Vec<Inner>,
}

#[derive(Content)]
struct Inner {
    inner: Vec<Item>,
}

#[derive(Content)]
struct Item {
    item: usize,
}

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut outer = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(Item { item: i });
        }
        outer.push(Inner { inner });
    }

    let tmpl = Template::new(BIG_TABLE_TEMPLATE).unwrap();
    let ctxt = Outer { outer };

    let _ = tmpl.render(&ctxt);
    b.iter(|| tmpl.render(&ctxt));
}

static BIG_TABLE_TEMPLATE: &'static str = "<html>
  {{#outer}}
    <tr>{{#inner}}<td>{{item}}</td>{{/inner}}</tr>
  {{/outer}}
</html>";

#[derive(Content)]
struct Teams<'a> {
    year: u16,
    teams: Vec<Team<'a>>,
}

#[derive(Content)]
struct Team<'a> {
    name: &'a str,
    score: u8,
    champ: bool,
}

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
    let tmpl = Template::new(TEAMS_TEMPLATE).unwrap();
    let ctxt = Teams {
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
    };

    let _ = tmpl.render(&ctxt);
    b.iter(|| tmpl.render(&ctxt));
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
