# P1328 — sucessor mecânico e RED R2 antes de C

Revisor `/root/p1328_review`; A/B sem atestação técnica de isolamento.
Somente diagnóstico escrito. Runtime continuava baseline nesta fase.

O diff integral `p1328-ab-tests-r1.rs` → `p1328-ab-tests-r2.rs` muda apenas
a ordenação/formatação dos mesmos quatro imports na primeira declaração
`use`. Nenhum assert, fixture, controle ou expectativa foi alterado.
Hashes conferidos: snippet R2
`b689de73f3bbe572ed4cc9a0fea0fc29a792f91a8125cc8e1a657a19e676315e`;
freeze R2 `6e1c5db370a44d1ba8f48bf07cbe0f0858cea65a4728e1db680affb9f12e4482`;
integração R2 `12657ee617b6d431e6b03750f503d7e69c72c5ebd71725434183c3fe2db302c6`.
Runner e expectativas R1 conservam os hashes congelados na revisão pré-C.

A fonte integrada SHA-256
`e2133de05b139fec2138c00857904c5ffdff1cd99fb6e34a7d056e4b75f219d1`
termina com os bytes exatos de R2; prefixo igual ao baseline original após
neutralizar somente `@prompt-hash`. `p1328-integrated-fmt-r2.json` registra
`cargo fmt --all -- --check`, exit 0, stdout/stderr vazios.

O RED R2 `p1328-unit-red-r2.json` tem SHA-256 verificado
`de861453a8b18d1638d71468b5cdc754a84090c3af7203a8baba69c578a790dd`;
manifesto `f7a0c4d6e7d6358fc5d34ff75465aca5da8d11188a2731e9275b9efbfe803098`.
Execução `cargo test --release --locked -p typst-core p1328 -- --nocapture`,
target `/tmp/p1328-target.T7Tg57`, de
`2026-09-09T11:51:23.818009+00:00` até
`2026-09-09T11:52:48.227681+00:00`, exit 101. HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093` mais working tree não commitado;
diff/stat e inventários completos constam no recibo. A fonte calc conserva
o hash integrado R2 em ambos inventários antes/depois.

Compilação concluída, binário unit executado: 7 testes, 5 falhas e 2 passes.
Todas as falhas são da mensagem antiga de Content contra o literal L0;
os grupos de guards/números e de rejeições preservadas passam. É o mesmo
vetor causal do RED R1, não falha de harness. Cada grupo falho interrompe
no primeiro assert; somente GREEN confirma execução integral das variantes.

Veredito: R2 mecânico, fmt válido e RED semântico confirmado. Os gates
pré-C da revisão estão satisfeitos para materializar a correção no owner
calc contra o L0 e estes artefatos congelados. Isso não aprova C nem
fechamento; GREEN, CLI integral e gates finais permanecem obrigatórios.
