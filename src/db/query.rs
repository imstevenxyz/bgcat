use std::collections::HashMap;
use surrealdb::engine::any::Any;
use surrealdb::{Error, Response, Surreal};

use crate::web::queries::BoardGameQuery;

pub struct DbQueryBuilder<'a> {
    pub segments: Vec<&'a str>,
    pub bindings: HashMap<String, serde_json::Value>,
}

impl<'a> DbQueryBuilder<'a> {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            bindings: HashMap::new(),
        }
    }

    pub fn select<'b>(&'b mut self, what: &'a str, from: &'a str) -> &'b mut Self {
        self.segments.extend(["SELECT", what, "FROM", from]);
        self
    }

    pub fn r#where<'b>(&'b mut self) -> &'b mut Self {
        self.segments.push("WHERE");
        self
    }

    pub fn condition<'b>(
        &'b mut self,
        condition: &'a str,
        bind: (&'a str, serde_json::Value),
    ) -> &'b mut Self {
        self.segments.push(condition);
        self.bindings
            .insert(bind.0.to_string().replace("$", ""), bind.1);
        self
    }

    pub fn and<'b>(&'b mut self) -> &'b mut Self {
        self.segments.push("AND");
        self
    }

    pub fn order_by<'b>(&'b mut self, field: &'a str, direction: &'a str) -> &'b mut Self {
        self.segments.extend(["ORDER", field, "NUMERIC", direction]);
        self
    }

    pub fn group_by<'b>(&'b mut self, field: &'a str) -> &'b mut Self {
        self.segments.extend(["GROUP", field]);
        self
    }

    pub fn paginate<'b>(&'b mut self, limit: u32, start: u32) -> &'b mut Self {
        self.segments.push("LIMIT $limit START $start");
        self.bindings.insert("limit".to_string(), limit.into());
        self.bindings.insert("start".to_string(), start.into());
        self
    }

    pub fn build(&self) -> DbQuery {
        let mut query = self.segments.join(" ");
        query.push_str(";");

        DbQuery {
            query,
            bindings: self.bindings.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DbQuery {
    query: String,
    bindings: HashMap<String, serde_json::Value>,
}

impl DbQuery {
    pub fn select(query: &BoardGameQuery) -> Self {
        let mut builder = DbQueryBuilder::new();
        builder.select("*", "boardgame");

        for (i, v) in generate_conditions(query).into_iter().enumerate() {
            if i == 0 {
                builder.r#where();
            } else {
                builder.and();
            }
            builder.condition(v.0, v.1);
        }

        let binding1 = query.sort_by.to_string();
        let binding2 = query.sort_direction.to_string();
        builder.order_by(&binding1, &binding2);

        if query.limit > 0 {
            builder.paginate(query.limit, query.page * query.limit);
        }

        builder.build()
    }

    pub fn count(query: &BoardGameQuery) -> Self {
        let mut builder = DbQueryBuilder::new();
        builder.select("count()", "boardgame");

        for (i, v) in generate_conditions(query).into_iter().enumerate() {
            if i == 0 {
                builder.r#where();
            } else {
                builder.and();
            }
            builder.condition(v.0, v.1);
        }

        // builder.group_by("ALL"); // Broken: https://github.com/surrealdb/surrealdb/issues/1786
        builder.group_by("count");

        builder.build()
    }

    pub async fn execute(&self, db: &Surreal<Any>) -> Result<Response, Error> {
        Ok(db.query(&self.query).bind(self.bindings.clone()).await?)
    }
}

fn generate_conditions<'a>(query: &BoardGameQuery) -> Vec<(&'a str, (&'a str, serde_json::Value))> {
    let mut conds: Vec<(&str, (&str, serde_json::Value))> = Vec::new();

    match &query.search {
        None => {}
        Some(val) => conds.push(("title ~ $title", ("$title", val.clone().into()))),
    }

    match &query.players {
        None => {}
        Some(val) => conds.extend([
            (
                "min_players <= $min_players",
                ("$min_players", val.clone().into()),
            ),
            (
                "(players_no_limit OR max_players >= $max_players)",
                ("$max_players", val.clone().into()),
            ),
        ]),
    }

    match &query.min_playtime {
        None => {}
        Some(val) => conds.push((
            "min_playtime >= $min_playtime",
            ("$min_playtime", val.clone().into()),
        )),
    }

    match &query.max_playtime {
        None => {}
        Some(val) => conds.push((
            "max_playtime <= $max_playtime",
            ("$max_playtime", val.clone().into()),
        )),
    }

    conds
}
