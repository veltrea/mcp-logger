use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use flexi_logger::{FileSpec, Logger, WriteMode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::io::{self, BufRead, IsTerminal, Write};

mod core_syslog;
use core_syslog::{Facility, Severity, send_syslog};

#[derive(Parser)]
#[command(name = "mcp-logger")]
#[command(version = "0.1.0")]
#[command(about = "A dual-purpose tool that works as an MCP server and a standalone CLI logger.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as an MCP server (JSON-RPC over stdio)
    RunMcp,
    /// Send a standalone syslog message (CLI mode)
    Send {
        /// Syslog server address (IP or hostname)
        server: String,
        /// Message to send
        message: String,
        /// Port number (default: 514)
        #[arg(short, long, default_value_t = 514)]
        port: u16,
        /// Facility (e.g., user, local0)
        #[arg(short, long, default_value = "user")]
        facility: String,
        /// Severity (e.g., info, error)
        #[arg(short, long, default_value = "info")]
        severity: String,
        /// Application tag
        #[arg(short, long, default_value = "mcp-logger")]
        tag: String,
        /// Character encoding (e.g., utf-8, shift_jis)
        #[arg(short, long, default_value = "utf-8")]
        encoding: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Value>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::RunMcp) => run_mcp_loop(),
        Some(Commands::Send {
            server,
            message,
            port,
            facility,
            severity,
            tag,
            encoding,
        }) => {
            let f = Facility::from_str(&facility);
            let s = Severity::from_str(&severity);
            let result = send_syslog(&server, port, f, s, &tag, &encoding, &message)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            println!("{}", result);
            Ok(())
        }
        None => {
            if io::stdin().is_terminal() {
                // Human is calling the tool without arguments
                print_mcp_setup_guide();
                Ok(())
            } else {
                // Pipe/AI is calling the tool without arguments
                run_mcp_loop()
            }
        }
    }
}

fn print_mcp_setup_guide() {
    println!("MCP Logger - AI-Friendly Syslog Client");
    println!("\nThis tool is designed to be used as an MCP server (Model Context Protocol).");
    println!(
        "To use it with AI assistants (e.g., Claude, Gemini via LM Studio), add it to your configuration."
    );
    println!("\n--- LM Studio / Claude Desktop Config Example ---");
    println!(
        r#""mcp-logger": {{
  "command": "{}",
  "args": ["run-mcp"]
}}"#,
        env::current_exe().unwrap_or_default().display()
    );
    println!("--------------------------------------------------");
    println!("\nUsage for humans:");
    println!("  mcp-logger send <SERVER> <MESSAGE> --port 514 --severity error");
    println!("\nRun 'mcp-logger --help' for more details.");
}

fn run_mcp_loop() -> Result<()> {
    // Initialize logging to a file since stdout is used for MCP communication
    let start_time = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let exe_path = env::current_exe().context("Failed to get exe path")?;
    let log_dir = exe_path.parent().context("Failed to get parent dir")?;

    Logger::try_with_str("info")?
        .log_to_file(
            FileSpec::default()
                .directory(log_dir)
                .basename("mcp-logger"),
        )
        .write_mode(WriteMode::Direct)
        .start()?;

    log::info!("MCP Logger Session Started: {}", start_time);

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut stdout = io::stdout();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
            match req.method.as_str() {
                "initialize" => {
                    let client_protocol_version = req
                        .params
                        .as_ref()
                        .and_then(|p| p.get("protocolVersion"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("2024-11-05");

                    let response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: Some(serde_json::json!({
                            "protocolVersion": client_protocol_version,
                            "capabilities": {
                                "tools": {}
                            },
                            "serverInfo": {
                                "name": "mcp-logger",
                                "version": "0.1.0"
                            }
                        })),
                        error: None,
                        id: req.id,
                    };
                    send_response(&mut stdout, response)?;
                }
                "notifications/initialized" => {}
                "tools/list" => {
                    let response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: Some(serde_json::json!({
                            "tools": [
                                {
                                    "name": "send-syslog",
                                    "description": "Send a standard Syslog message to a generic Syslog receiver (UDP 514).",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "server": {
                                                "type": "string",
                                                "description": "IP address or hostname of the Syslog server."
                                            },
                                            "port": {
                                                "type": "integer",
                                                "description": "Port number of the Syslog server.",
                                                "default": 514
                                            },
                                            "facility": {
                                                "type": "string",
                                                "description": "Syslog functionality code (e.g., 'user', 'local0').",
                                                "default": "user"
                                            },
                                            "severity": {
                                                "type": "string",
                                                "description": "Severity level (e.g., 'info', 'error').",
                                                "default": "info"
                                            },
                                            "tag": {
                                                "type": "string",
                                                "description": "Tag or application name.",
                                                "default": "mcp-syslog"
                                            },
                                            "encoding": {
                                                "type": "string",
                                                "description": "Character encoding (e.g., 'utf-8', 'shift_jis').",
                                                "default": "utf-8"
                                            },
                                            "message": {
                                                "type": "string",
                                                "description": "The actual log message text."
                                            }
                                        },
                                        "required": ["server", "message"]
                                    }
                                }
                            ]
                        })),
                        error: None,
                        id: req.id,
                    };
                    send_response(&mut stdout, response)?;
                }
                "tools/call" => {
                    if let Some(params) = req.params {
                        let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                        let args = params.get("arguments");

                        if name == "send-syslog" {
                            if let Some(args) = args {
                                let server =
                                    args.get("server").and_then(|s| s.as_str()).unwrap_or("");
                                let message =
                                    args.get("message").and_then(|m| m.as_str()).unwrap_or("");

                                if server.is_empty() || message.is_empty() {
                                    send_error(
                                        &mut stdout,
                                        req.id,
                                        -32602,
                                        "Missing required arguments 'server' or 'message'",
                                    )?;
                                    continue;
                                }

                                let port =
                                    args.get("port")
                                        .and_then(|p| p.get("port"))
                                        .and_then(|v| v.as_u64())
                                        .or_else(|| args.get("port").and_then(|v| v.as_u64()))
                                        .unwrap_or(514) as u16;

                                let facility = Facility::from_str(
                                    args.get("facility")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("user"),
                                );
                                let severity = Severity::from_str(
                                    args.get("severity")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("info"),
                                );
                                let tag = args
                                    .get("tag")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("mcp-syslog");
                                let encoding = args
                                    .get("encoding")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("utf-8");

                                match send_syslog(
                                    server, port, facility, severity, tag, encoding, message,
                                ) {
                                    Ok(result_msg) => {
                                        let response = JsonRpcResponse {
                                            jsonrpc: "2.0".to_string(),
                                            result: Some(serde_json::json!({
                                                "content": [
                                                    {
                                                        "type": "text",
                                                        "text": result_msg
                                                    }
                                                ],
                                                "isError": false
                                            })),
                                            error: None,
                                            id: req.id,
                                        };
                                        send_response(&mut stdout, response)?;
                                    }
                                    Err(e) => {
                                        let response = JsonRpcResponse {
                                            jsonrpc: "2.0".to_string(),
                                            result: Some(serde_json::json!({
                                                "content": [
                                                    {
                                                        "type": "text",
                                                        "text": format!("Failed to send syslog: {}", e)
                                                    }
                                                ],
                                                "isError": true
                                            })),
                                            error: None,
                                            id: req.id,
                                        };
                                        send_response(&mut stdout, response)?;
                                    }
                                }
                            } else {
                                send_error(&mut stdout, req.id, -32602, "Missing arguments")?;
                            }
                        } else {
                            send_error(
                                &mut stdout,
                                req.id,
                                -32601,
                                &format!("Tool not found: {}", name),
                            )?;
                        }
                    } else {
                        send_error(&mut stdout, req.id, -32602, "Invalid params")?;
                    }
                }
                "ping" => {
                    let response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: Some(serde_json::json!({})),
                        error: None,
                        id: req.id,
                    };
                    send_response(&mut stdout, response)?;
                }
                _ => {
                    if req.id.is_some() {
                        send_error(&mut stdout, req.id, -32601, "Method not found")?;
                    }
                }
            }
        }
    }

    log::info!("MCP Logger Session Ended");
    Ok(())
}

fn send_response(stdout: &mut io::Stdout, response: JsonRpcResponse) -> Result<()> {
    let resp_str = serde_json::to_string(&response)?;
    stdout.write_all(resp_str.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

fn send_error(stdout: &mut io::Stdout, id: Option<Value>, code: i32, message: &str) -> Result<()> {
    let response = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(serde_json::json!({
            "code": code,
            "message": message
        })),
        id,
    };
    send_response(stdout, response)
}
