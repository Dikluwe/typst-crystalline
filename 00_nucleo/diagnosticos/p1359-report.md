# P1359 — relatório de verificação pós-implementação estática

## Veredito

**PASS estático restrito.** A implementação atual em L2 e L4 materializa o
contrato tri-state de `--pdf-tagged`, os inputs protegidos permanecem íntegros e
o safety gate selado rejeita 14/14 mutantes (`mutation_score = 1.0`). Este
veredito não declara equivalência funcional geral nem substitui uma matriz
bilateral dinâmica completa.

Regime: protocolo completo de materialização segregada, executado sem
atestação de isolamento. Papel: verificador pós-implementação estático
`/root/verificacao_estatica_p1359`. O uso da skill tornou explícita a separação
entre inspeção atual, evidência reutilizada e limitações de atestação.

## Proibição de reexecução e proveniência

Houve **zero reexecução pós-intervenção por esta autoridade**: nenhum `cargo`,
teste, filtro `p1359_`, binário `typst`/candidato, `watch`, `w` ou subprocesso
de produto foi iniciado. O safety gate não foi reexecutado, pois a tarefa
manda reutilizá-lo e não executar qualquer caminho com risco de spawn.

A inspeção ocorreu em `2026-09-13T01:51:11Z`, no HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248` (tree
`c3c2fa834e6ec318690ddb88e181277ff2e25b27`), com working tree não commitado.
No recorte P1359, `git diff HEAD --stat` mostrou 5 ficheiros, 259 inserções e 6
remoções: os dois L0 owners, os dois consumers produtivos e a suíte CLI de L4.

## Inspeção estática atual

- `02_shell/src/cli.rs` declara `pdf_tagged: Option<bool>` com
  `num_args = 0..=1`, `default_missing_value = "true"` e
  `require_equals = true`, e transporta o valor sem colapsá-lo em
  `CompileIntent`.
- O teste P1359 de L2 cobre `compile`, `watch` e `w` somente com
  `Args::try_parse_from`, no mesmo processo. Não há spawn para V05–V07.
- `04_wiring/src/main.rs` resolve a precedência exatamente como o contrato:
  `Some(true) → Enabled`, `Some(false) → Disabled`, `None` com legado →
  `Disabled`, e ausência de ambos → `Enabled`.
- `pdf_tags` só entra nos braços PDF. PNG, SVG e HTML não recebem esse valor;
  `stream_mode` continua sendo resolvido separadamente.
- Todos os `Command::new` adicionados na secção P1359 de
  `04_wiring/tests/cli.rs` invocam apenas `compile`, um comando finito. Não há
  processo P1359 `watch` ou `w` nessa suíte.
- Os headers de linhagem apontam aos owners corretos. A autoridade
  coordenadora `/root` informou gates atuais com exit 0 para
  `cargo fmt --all -- --check`, `git diff --check` e `crystalline-lint .`; o
  último reportou zero violations. Esses gates não foram executados por este
  verificador.

## Integridade dos inputs protegidos

Os hashes atuais conferem com o manifesto e o safety seal:

| Artefato | SHA-256 |
|---|---|
| `p1359-manifest.json` | `acedfffe1bb426855d542dc9e0e36cdc99a61c20efd9ad2b6f7aa6802234e64d` |
| `p1359-contract.json` | `a6004e2e759048a8651abe54608110b7b8812cb38b200df58abe5e58a299aedd` |
| `p1359-oracles.json` | `eb1846a8b9a612227af31a3a6c15d474d1f461ec4d5991f02952f7c69ea07e50` |
| `p1359-mutants.json` | `7a8eb96dfb45b3c55cc49f7449edef9c4a5a4a11aaabea7c12de41cb629eb10b` |
| `p1359-safety-seal.json` | `523acf9e2236f449dd510eba40d46b6829fe4a00d2e4c39df29da3ed339b0db7` |
| `p1359-safety-gate.py` | `fccfe7e8e468816bf9ce6a5e86ac4804b78121f907992a1067c0825301e08635` |
| `typst-passo-1359.md` | `9b9904c638e14fe21d04820807e53396dbeb6b7b3dbba18a1eaaf52973d0fc4d` |
| `prompts/shell/cli.md` | `37dbd1947ba6e1d89eb3598c548401ac922e3028a347e6153dbdc674725393ac` |
| `prompts/wiring.md` | `5d4dd2773b24f919ff45306bac6a4963eaf1b0683d92cadf2e8c15d136f16514` |

Hashes dos outputs candidatos inspecionados:

| Artefato | SHA-256 |
|---|---|
| `02_shell/src/cli.rs` | `704d2eeb4f2742cad8d874a6d1b431af42124afeaa57e2efeab5bfaabb7ae211` |
| `04_wiring/src/main.rs` | `e9148acec9ff35df0bee6301abe9d232b7b4683165ffd67a599dc4893a8cd3ad` |
| `04_wiring/tests/cli.rs` | `59c66cc635bbeaeaf8aef6789aaf2cd451121e8533935b883e3929bb9c683375` |

## Evidência reutilizada, sem reexecução

O recibo `/tmp/p1359-safety-gate-result.json`, SHA-256
`c68824b145e9bbabfa2a60625c2dd04f068d080ee1285554eb1ed5eccd3cf45b`,
regista PASS, 14 mutantes válidos, 14 rejeitados, score 1.0, A14 rejeitado
antes de execução, zero processo lançado e determinismo em quatro ordenações.
O recibo de proveniência
`/tmp/p1359-safety-provenance.json`, SHA-256
`b6adb6ad01de678e029b56e085fc75e006f65d587a0a850db1ed030668297a6b`,
vincula corretamente o manifesto, gate, selo e recibo aos hashes atuais.

Os recibos GREEN existentes foram apenas lidos:

| Recibo | SHA-256 | Conteúdo observado |
|---|---|---|
| `/tmp/p1359-green-shell-focal.txt` | `116394bbcaaeba1d6976b2f405c0ade35d4ffdf47418ed4e9c425749bef9000a` | 2 passed, 0 failed, 62 filtered out |
| `/tmp/p1359-green-shell-suite.txt` | `35753095facb5a63a6ffbf6fefab1644bf8d53b7240e9bedeb42184979f33215` | 64 passed, 0 failed |
| `/tmp/p1359-green-wiring-focal.txt` | `864a862ca72a8165136f2e836e8bc92e75acf4ef934ac13f303d5b3ad68c49a0` | 4 passed, 0 failed, 71 filtered out |
| `/tmp/p1359-green-wiring-suite.txt` | `e74a4aa06bbaed4a07833e31a7e1c275652827d7399295d8ee74b5b04fd893d4` | todos os grupos registados passaram; os 4 testes P1359 aparecem como `ok` |
| `/tmp/p1359-fmt-check.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | captura vazia |

`/tmp/p1359-post-contract-gate.json` (SHA-256
`75a08186bd5d4b106fcc296498493ba6fbe71235ec131c54d73d898815eaab57`)
não foi usado para a revisão atual: ele está vinculado à revisão superseded de
13 mutantes e a hashes predecessores. O safety receipt de 14/14 é a evidência
estática vigente.

## Limites do veredito

Os recibos GREEN são evidência reutilizada e seus arquivos de texto não
incluem o comando invocador nem hashes do candidato, portanto não são
autossuficientes para reprodução. Além disso, o teste P1359 de wiring procura
tokens estruturais diretamente no PDF e cobre neutralidade não-PDF apenas para
PNG; ele não constitui por si só o recibo do observador qpdf bilateral nos
quatro perfis, incluindo SVG e HTML, exigido pelos oráculos selados.

Assim, a conclusão segura é: implementação e contenção de processos conformes
por inspeção estática; inputs protegidos íntegros; safety mutation gate 14/14
válido como evidência reutilizada; verificação dinâmica pós-intervenção não
executada por restrição expressa; nenhuma alegação de equivalência geral.
