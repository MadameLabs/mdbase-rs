use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

fn run(name: &str, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_mdbase-local"))
        .arg("-C")
        .arg(fixture(name))
        .args(args)
        .output()
        .expect("mdbase-local roda");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.code().unwrap_or(-1), text)
}

#[test]
fn pessoa_ok_valida() {
    let (code, out) = run("pessoa-ok", &["validate"]);
    assert_eq!(code, 0, "{out}");
}

#[test]
fn pessoa_sem_nome_reprova() {
    let (code, out) = run("pessoa-sem-nome", &["validate"]);
    assert_ne!(code, 0, "{out}");
    assert!(out.contains("name"), "{out}");
}

#[test]
fn mapeamento_errado_reprova() {
    let (code, out) = run("mapeamento-errado", &["validate"]);
    assert_ne!(code, 0, "{out}");
    assert!(out.contains("data_contract_field_invalid"), "{out}");
}

#[test]
fn pasta_de_config_e_lida() {
    let (code, out) = run("pessoa-ok", &["types", "list"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("person"), "{out}");
}
