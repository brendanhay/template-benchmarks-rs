use criterion;
use minijinja::{context, Environment};
use serde::Serialize;

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }

    let mut env = Environment::new();
    env.add_template("big-table.html", BIG_TABLE_TEMPLATE)
        .unwrap();
    let tmpl = env.get_template("big-table.html").unwrap();
    let ctxt = context!(table => &table);

    let _ = tmpl.render(&ctxt).unwrap();
    b.iter(|| tmpl.render(&ctxt));
}

static BIG_TABLE_TEMPLATE: &'static str = "<table>
  {% for row in table %}
    <tr>{% for col in row %}<td>{{ col }}</td>{% endfor %}</tr>
  {% endfor %}
</table>";

#[derive(Serialize)]
struct Team {
    name: String,
    score: u8,
}

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
    let mut env = Environment::new();
    env.add_template("teams.html", TEAMS_TEMPLATE).unwrap();
    let tmpl = env.get_template("teams.html").unwrap();
    let ctxt = context! {
      year => 2015,
      teams => vec![
             Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
      ]
    };

    let _ = tmpl.render(&ctxt).unwrap();
    b.iter(|| tmpl.render(&ctxt));
}

static TEAMS_TEMPLATE: &'static str = r#"<html>
  <head>
    <title>{{ year }}</title>
  </head>
  <body>
    <h1>CSL {{ year }}</h1>
    <ul>
    {% for team in teams %}
      <li class="{% if loop.first %}champion{% endif %}">
      <b>{{team.name}}</b>: {{team.score}}
      </li>
    {% endfor %}
    </ul>
  </body>
</html>"#;
