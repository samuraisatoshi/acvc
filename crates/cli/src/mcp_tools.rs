use std::process::Command;

use acvc_domain::{Error, Result};
use serde_json::{Map, Value, json};

struct Field {
    name: &'static str,
    flag: &'static str,
    kind: Kind,
}

enum Kind {
    Optional,
    Repeated,
    Positional,
    OptionalPositional,
}

struct Tool {
    name: &'static str,
    description: &'static str,
    command: &'static [&'static str],
    fields: &'static [Field],
}

pub fn list_tools() -> Value {
    json!({ "tools": TOOLS.iter().map(schema).collect::<Vec<_>>() })
}

pub fn call(name: &str, arguments: &Value) -> Result<(bool, String)> {
    let tool = TOOLS
        .iter()
        .find(|tool| tool.name == name)
        .ok_or_else(|| Error::new("unknown tool").ctx("tool", name))?;
    let input = match arguments {
        Value::Object(map) => map,
        Value::Null => &Map::new(),
        _ => return Err(Error::new("tool arguments must be an object").ctx("tool", name)),
    };
    let argv = argv_of(tool, input)?;
    let exe =
        std::env::current_exe().map_err(|err| Error::new(err.to_string()).ctx("op", "exe"))?;
    let mut command = Command::new(exe);
    if let Some(repo) = optional(input, "repo")? {
        command.arg("--repo").arg(repo);
    }
    command.args(argv);
    let output = command.output().map_err(|err| {
        Error::new(err.to_string())
            .ctx("op", "spawn")
            .ctx("tool", name)
    })?;
    let text = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        String::from_utf8_lossy(&output.stderr).into_owned()
    };
    Ok((!output.status.success(), text))
}

fn argv_of(tool: &Tool, input: &Map<String, Value>) -> Result<Vec<String>> {
    let mut argv: Vec<String> = tool
        .command
        .iter()
        .map(|part| (*part).to_string())
        .collect();
    for field in tool.fields {
        match field.kind {
            Kind::Optional | Kind::Positional | Kind::OptionalPositional => {
                let Some(value) = optional(input, field.name)? else {
                    continue;
                };
                if !field.flag.is_empty() {
                    argv.push(field.flag.to_string());
                }
                argv.push(value);
            }
            Kind::Repeated => {
                for value in repeated(input, field.name)? {
                    argv.push(field.flag.to_string());
                    argv.push(value);
                }
            }
        }
    }
    Ok(argv)
}

fn optional(input: &Map<String, Value>, name: &str) -> Result<Option<String>> {
    match input.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(Error::new("argument must be a string").ctx("field", name)),
    }
}

fn repeated(input: &Map<String, Value>, name: &str) -> Result<Vec<String>> {
    match input.get(name) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| match item {
                Value::String(value) => Ok(value.clone()),
                _ => Err(Error::new("argument must be a string").ctx("field", name)),
            })
            .collect(),
        Some(_) => Err(Error::new("argument must be an array").ctx("field", name)),
    }
}

fn schema(tool: &Tool) -> Value {
    let mut properties = Map::new();
    properties.insert(
        "repo".into(),
        json!({"type": "string", "description": "Repositório ACVC"}),
    );
    let mut required = Vec::new();
    for field in tool.fields {
        let (type_name, items) = match field.kind {
            Kind::Repeated => ("array", Some(json!({"type": "string"}))),
            Kind::Positional => {
                required.push(field.name);
                ("string", None)
            }
            Kind::Optional | Kind::OptionalPositional => ("string", None),
        };
        let mut spec = json!({"type": type_name});
        if let Some(items) = items {
            spec["items"] = items;
        }
        properties.insert(field.name.into(), spec);
    }
    json!({
        "name": tool.name,
        "description": tool.description,
        "inputSchema": {"type": "object", "properties": properties, "required": required}
    })
}

const fn opt(name: &'static str, flag: &'static str) -> Field {
    Field {
        name,
        flag,
        kind: Kind::Optional,
    }
}

const fn rep(name: &'static str, flag: &'static str) -> Field {
    Field {
        name,
        flag,
        kind: Kind::Repeated,
    }
}

const fn pos(name: &'static str) -> Field {
    Field {
        name,
        flag: "",
        kind: Kind::Positional,
    }
}

const fn pos_opt(name: &'static str) -> Field {
    Field {
        name,
        flag: "",
        kind: Kind::OptionalPositional,
    }
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "init",
        description: "Cria um repositório ACVC",
        command: &["init"],
        fields: &[],
    },
    Tool {
        name: "campaign_create",
        description: "Cria uma campanha",
        command: &["campaign", "create"],
        fields: &[pos("name"), rep("metrics", "--metric")],
    },
    Tool {
        name: "campaign_list",
        description: "Lista campanhas",
        command: &["campaign", "list"],
        fields: &[],
    },
    Tool {
        name: "seed",
        description: "Grava a primeira variante de uma pasta",
        command: &["seed"],
        fields: &[
            opt("campaign", "--campaign"),
            opt("path", "--path"),
            opt("premise", "--premise"),
            rep("assumptions", "--assumption"),
            opt("intent", "--intent"),
            opt("agent", "--agent"),
        ],
    },
    Tool {
        name: "materialize",
        description: "Materializa uma variante numa pasta",
        command: &["materialize"],
        fields: &[opt("variant", "--variant"), opt("into", "--into")],
    },
    Tool {
        name: "status",
        description: "Mostra o estado da pasta de trabalho",
        command: &["status"],
        fields: &[opt("workspace", "--workspace")],
    },
    Tool {
        name: "record",
        description: "Grava uma variante a partir da pasta",
        command: &["record"],
        fields: &[
            opt("workspace", "--workspace"),
            opt("edge", "--edge"),
            opt("premise", "--premise"),
            rep("assumptions", "--assumption"),
            opt("revises", "--revises"),
            opt("intent", "--intent"),
            opt("agent", "--agent"),
            rep("parents", "--parent"),
        ],
    },
    Tool {
        name: "score",
        description: "Regista medições de uma variante",
        command: &["score"],
        fields: &[opt("variant", "--variant"), rep("metrics", "--metric")],
    },
    Tool {
        name: "reject",
        description: "Rejeita uma variante",
        command: &["reject"],
        fields: &[opt("variant", "--variant"), opt("reason", "--reason")],
    },
    Tool {
        name: "compare",
        description: "Compara duas variantes",
        command: &["compare"],
        fields: &[pos("left"), pos("right")],
    },
    Tool {
        name: "file_diff",
        description: "Mostra o diff de um arquivo entre duas variantes",
        command: &["file-diff"],
        fields: &[pos("left"), pos("right"), opt("path", "--path")],
    },
    Tool {
        name: "frontier",
        description: "Lista a fronteira de Pareto",
        command: &["frontier"],
        fields: &[opt("campaign", "--campaign")],
    },
    Tool {
        name: "recall",
        description: "Busca variantes pelo texto",
        command: &["recall"],
        fields: &[opt("campaign", "--campaign"), opt("query", "--query")],
    },
    Tool {
        name: "why",
        description: "Mostra premissa, pais e trials",
        command: &["why"],
        fields: &[pos_opt("variant")],
    },
    Tool {
        name: "backtrack",
        description: "Volta ao pai de uma variante",
        command: &["backtrack"],
        fields: &[pos("variant"), opt("into", "--into")],
    },
    Tool {
        name: "pack",
        description: "Empacota blobs frios",
        command: &["pack"],
        fields: &[],
    },
    Tool {
        name: "serve",
        description: "Publica o repositório no protocolo remoto",
        command: &["serve"],
        fields: &[opt("bind", "--bind")],
    },
    Tool {
        name: "push",
        description: "Envia uma variante para outro repositório",
        command: &["push"],
        fields: &[
            opt("to", "--to"),
            opt("variant", "--variant"),
            opt("campaign", "--campaign"),
        ],
    },
    Tool {
        name: "fetch",
        description: "Recebe uma variante de outro repositório",
        command: &["fetch"],
        fields: &[
            opt("from", "--from"),
            opt("variant", "--variant"),
            opt("campaign", "--campaign"),
        ],
    },
    Tool {
        name: "promote",
        description: "Exporta uma variante para uma pasta ou um commit git",
        command: &["promote"],
        fields: &[
            opt("variant", "--variant"),
            opt("dest", "--dest"),
            opt("git", "--git"),
        ],
    },
    Tool {
        name: "tui",
        description: "Abre o visor no terminal",
        command: &["tui"],
        fields: &[],
    },
    Tool {
        name: "web",
        description: "Abre a página da campanha",
        command: &["web"],
        fields: &[opt("bind", "--bind")],
    },
    Tool {
        name: "obsidian",
        description: "Instala o visor no vault e aponta o repositório. Sem copiar o plugin à mão.",
        command: &["obsidian"],
        fields: &[opt("vault", "--vault")],
    },
    Tool {
        name: "context",
        description: "Lê a campanha antes de agir: problema, necessidade, agente, interações, tokens, idade e o mapa das ramificações",
        command: &["context"],
        fields: &[opt("campaign", "--campaign")],
    },
    Tool {
        name: "guide",
        description: "Como usar o ACVC: o que uma hipótese precisa dizer, quando chamar cada comando, exemplo",
        command: &["guide"],
        fields: &[],
    },
    Tool {
        name: "assistant",
        description: "Devolve o contrato operacional no idioma do computador. Quem tem o MCP do ASB envia documents para rag.index",
        command: &["assistant"],
        fields: &[opt("part", "--part")],
    },
    Tool {
        name: "spend",
        description: "Registra tokens consumidos na campanha sem mudar a identidade da hipótese",
        command: &["spend"],
        fields: &[
            opt("campaign", "--campaign"),
            opt("tokens", "--tokens"),
            opt("note", "--note"),
            opt("agent", "--agent"),
        ],
    },
];
