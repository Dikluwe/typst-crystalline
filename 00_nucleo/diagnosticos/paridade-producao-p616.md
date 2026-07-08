# Paridade de produção — P616: Rejeitar `dir: ttb`/`btt` em texto

**Data:** 2026-07-08  
**Hash do commit de fecho:** `3f36393c2`  
**Materialization:** `00_nucleo/materialization/typst-passo-616.md`

---

## 1. Contexto

Em P576, o cristalino passou a suportar `#set text(dir: ...)` para `ltr` e
`rtl`. No entanto, também aceitava silenciosamente `ttb` e `btt`, renderizando
o texto horizontalmente sem avisar o utilizador. P614 confirmou que o vanilla
0.15.0 rejeita esses valores.

## 2. Problema

Aceitar `dir: ttb`/`btt` em texto era um erro engolido: o utilizador pensa que
está a pedir escrita vertical, mas o compilador ignora a direcção. O vanilla
0.15.0 emite um erro hard claro.

Isto é diferente de "escrita vertical está ausente" — é uma disparidade de
validação que deve ser corrigida independentemente de quando (ou se) a escrita
vertical vier a ser implementada.

## 3. Decisão

Reproduzir o comportamento do vanilla 0.15.0: em `#set text(...)`, a propriedade
`dir` só aceita direcções horizontais (`ltr`, `rtl`). `ttb` e `btt` devolvem o
erro `"text direction must be horizontal"`, com span apontando para o valor do
argumento.

## 4. Implementação

### 4.1 Ficheiros alterados

- `00_nucleo/prompts/rules/eval.md`:
  - Adicionada secção §P616 com medição na fonte vanilla
    (`text/mod.rs:1258-1259`, `bidi.typ:70`) e regra de validação.
- `01_core/src/rules/eval/rules.rs`:
  - No arm `"dir"` de `#set text(...)`, verifica `dir.is_vertical()` antes de
    propagar o valor para a chain.
  - Devolve `SourceDiagnostic::error` com a mensagem do vanilla quando
    `Dir::TTB` ou `Dir::BTT`.
  - Remove import `crate::entities::dir::Dir` que ficou redundante.
- `04_wiring/tests/cli.rs`:
  - Adicionados três testes de integração:
    - `p616_text_dir_ttb_rejeitado`
    - `p616_text_dir_btt_rejeitado`
    - `p616_text_dir_rtl_continua_funcionar`
- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - Actualizada entrada "Escrita vertical CJK" para reflectir que o vanilla
    também rejeita `dir: ttb/btt` em texto; trata-se de funcionalidade nova,
    não disparidade.
- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - Actualizado estado da escrita vertical CJK com a mesma conclusão.
- Hashes de linhagem (`@prompt-hash`) dos ficheiros `rules/eval/*.rs` foram
  actualizados automaticamente por `crystalline-lint --fix-hashes`.

## 5. Validação

### 5.1 Testes automáticos

```bash
cargo test --workspace
```

Resultado: todos os testes passam, incluindo os três novos testes de CLI.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 5.2 Verificação manual (reproduzível)

```bash
cat > /tmp/p616-ttb.typ <<'EOF'
#set text(dir: ttb)
Texto.
EOF
./target/release/typst /tmp/p616-ttb.typ /tmp/p616.pdf
echo "Exit code: $?"
```

Resultado: exit code 1, stderr contém `error: text direction must be horizontal`.

```bash
cat > /tmp/p616-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 20pt)
مرحبا
EOF
./target/release/typst /tmp/p616-rtl.typ /tmp/p616-rtl.pdf
```

Resultado: PDF gerado sem regressão na sequência RTL.

## 6. Conclusão

P616 está concluído. O cristalino agora rejeita `dir: ttb`/`btt` em `#set
text(...)` com a mesma mensagem do vanilla 0.15.0, enquanto preserva o
funcionamento de `ltr`/`rtl`. A escrita vertical CJK deixa de ser registada
como disparidade e passa a ser classificada como funcionalidade nova ausente
em ambos os projectos.

---

## 7. Ligações

- Commit de fecho: `3f36393c2`
- Prompt L0: `00_nucleo/prompts/rules/eval.md`
- Materialization: `00_nucleo/materialization/typst-passo-616.md`
- Relatório de fundo: `00_nucleo/diagnosticos/paridade-producao-p614.md`
